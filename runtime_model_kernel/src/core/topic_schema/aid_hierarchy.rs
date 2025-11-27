use crate::IdGen;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::rc::Rc;
use watchmen_model::{TopicData, TopicDataValue, VoidR};

const MY_AID_ID: &'static str = "aid_me";
const AID_ROOT: &'static str = "aid_root";

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Ancestor {
    name: String,
    aid_id: String,
}

pub struct HierarchyAid;

impl HierarchyAid {
    pub fn new() -> Self {
        HierarchyAid {}
    }

    /// apply ancestor aid id to given data.\n
    ///    if not applied(used keys is 0), apply as to root, use "aid_root" as name,\n
    ///    use "aid_{ancestor.name}" as name when this name is not used,\n
    ///    use "aid_{ancestor.name}_{distance_to_ancestor}" as name when name in step b is used.
    ///    distance is the difference value of my hierarchy number and ancestor's hierarchy number
    ///
    ///    for example:
    ///    root.a.b.c.b.e, now data is e, will create the following aid properties:\n
    ///    aid_root: to root,\n
    ///    aid_a: to a,\n
    ///    aid_b: to b,\n
    ///    aid_c: to c,\n
    ///    aid_b_1: to b which is closer to me(e).
    /// given data should be modified
    fn apply_ancestor_aid_id(
        &self,
        data: &mut TopicData,
        my_hierarchy_number: usize,
        ancestor: &Rc<Ancestor>,
        used_ancestor_keys: &mut HashMap<String, bool>,
    ) {
        let used_count = used_ancestor_keys.len();
        let name = if used_count == 0 {
            AID_ROOT
        } else {
            let name = &ancestor.name;
            &*format!("aid_{}", name)
        };
        let name = if used_ancestor_keys.contains_key(name) {
            &*format!("{}_{}", name, my_hierarchy_number - used_count)
        } else {
            name
        };

        data.insert(
            name.to_string(),
            TopicDataValue::Str(ancestor.aid_id.clone()),
        );
        used_ancestor_keys.insert(name.to_string(), true);
    }

    fn append_ancestor(
        &self,
        ancestors: &Vec<Rc<Ancestor>>,
        name: String,
        aid_id: u128,
    ) -> Vec<Rc<Ancestor>> {
        let mut new_ancestors = ancestors.clone();
        new_ancestors.push(Rc::new(Ancestor {
            name,
            aid_id: aid_id.to_string(),
        }));
        new_ancestors
    }

    /// given data should be modified
    fn do_aid(&self, data: &mut TopicData, ancestors: &Vec<Rc<Ancestor>>) -> VoidR {
        // create aid me
        let aid_id_of_me = IdGen::next_id()?;
        data.insert(
            MY_AID_ID.to_string(),
            TopicDataValue::Str(aid_id_of_me.to_string()),
        );

        // create ancestor aid ids
        let my_hierarchy_number = ancestors.len();
        let mut used_ancestor_keys = HashMap::new();
        ancestors.into_iter().for_each(|ancestor| {
            self.apply_ancestor_aid_id(
                data,
                my_hierarchy_number,
                ancestor,
                &mut used_ancestor_keys,
            );
        });

        for (name, value) in data.iter_mut() {
            match value {
                TopicDataValue::Map(map) => self.do_aid(
                    map,
                    &self.append_ancestor(ancestors, name.clone(), aid_id_of_me),
                )?,
                TopicDataValue::Vec(vec) => {
                    let ancestors_of_sub =
                        self.append_ancestor(ancestors, name.clone(), aid_id_of_me);
                    for row in vec.iter_mut() {
                        match row {
                            TopicDataValue::Map(map) => self.do_aid(map, &ancestors_of_sub)?,
                            // TIP otherwise do nothing, even if it is a vec
                            _ => {}
                        }
                    }
                }
                // otherwise do nothing
                _ => {}
            }
        }
        Ok(())
    }

    pub fn aid(&self, data: &mut TopicData) -> VoidR {
        self.do_aid(data, &vec![])
    }
}
