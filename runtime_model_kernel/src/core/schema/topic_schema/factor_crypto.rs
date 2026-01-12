use crate::{
    AesCrypto, AesCryptoBuilder, CenterCharsMask, Crypto, DateMask, LastCharsMask, MailMask,
    Md5Crypto, Sha256Crypto,
};
use elf_base::StdR;
use elf_model::{FactorEncryptMethod, TenantId, TopicDataValue};
use std::sync::Arc;

pub enum FactorCrypto {
    Aes256Pkcs5Padding(AesCrypto),
    Md5(Md5Crypto),
    Sha256(Sha256Crypto),
    MaskMail(MailMask),
    MaskCenter3(CenterCharsMask),
    MaskCenter5(CenterCharsMask),
    MaskLast3(LastCharsMask),
    MaskLast6(LastCharsMask),
    MaskDay(DateMask),
    MaskMonth(DateMask),
    MaskMonthDay(DateMask),
}

impl FactorCrypto {
    pub fn get(
        encrypt_method: &FactorEncryptMethod,
        tenant_id: &Arc<TenantId>,
    ) -> StdR<Option<FactorCrypto>> {
        let crypto = match encrypt_method {
            FactorEncryptMethod::Aes256Pkcs5Padding => {
                FactorCrypto::Aes256Pkcs5Padding(AesCryptoBuilder::get(tenant_id)?)
            }
            FactorEncryptMethod::Md5 => FactorCrypto::Md5(Md5Crypto::new()),
            FactorEncryptMethod::Sha256 => FactorCrypto::Sha256(Sha256Crypto::new()),
            FactorEncryptMethod::MaskCenter3 => {
                FactorCrypto::MaskCenter3(CenterCharsMask::center_3())
            }
            FactorEncryptMethod::MaskCenter5 => {
                FactorCrypto::MaskCenter5(CenterCharsMask::center_5())
            }
            FactorEncryptMethod::MaskLast3 => FactorCrypto::MaskLast3(LastCharsMask::last_3()),
            FactorEncryptMethod::MaskLast6 => FactorCrypto::MaskLast6(LastCharsMask::last_6()),
            FactorEncryptMethod::MaskMail => FactorCrypto::MaskMail(MailMask::new()),
            FactorEncryptMethod::MaskDay => FactorCrypto::MaskDay(DateMask::day_of_month()),
            FactorEncryptMethod::MaskMonth => FactorCrypto::MaskMonth(DateMask::month()),
            FactorEncryptMethod::MaskMonthDay => {
                FactorCrypto::MaskMonthDay(DateMask::month_and_day())
            }
            FactorEncryptMethod::None => return Ok(None),
        };

        Ok(Some(crypto))
    }

    fn as_encryptor(&self) -> &dyn Crypto {
        match self {
            FactorCrypto::Aes256Pkcs5Padding(e) => e,
            FactorCrypto::Md5(e) => e,
            FactorCrypto::Sha256(e) => e,
            FactorCrypto::MaskMail(e) => e,
            FactorCrypto::MaskCenter3(e) | FactorCrypto::MaskCenter5(e) => e,
            FactorCrypto::MaskLast3(e) | FactorCrypto::MaskLast6(e) => e,
            FactorCrypto::MaskDay(e)
            | FactorCrypto::MaskMonth(e)
            | FactorCrypto::MaskMonthDay(e) => e,
        }
    }

    pub fn is_encrypted(&self, value: &TopicDataValue) -> bool {
        self.as_encryptor().is_encrypted(value)
    }

    pub fn encrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        self.as_encryptor().encrypt(value)
    }

    pub fn decrypt(&self, value: &TopicDataValue) -> StdR<Option<TopicDataValue>> {
        self.as_encryptor().decrypt(value)
    }
}
