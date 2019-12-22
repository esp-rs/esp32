#[doc = "Reader of register EFUSE_BLK0_RDATA0_REG"]
pub type R = crate::R<u32, super::EFUSE_BLK0_RDATA0_REG>;
#[doc = "Writer for register EFUSE_BLK0_RDATA0_REG"]
pub type W = crate::W<u32, super::EFUSE_BLK0_RDATA0_REG>;
#[doc = "Register EFUSE_BLK0_RDATA0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::EFUSE_BLK0_RDATA0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `EFUSE_RD_FLASH_CRYPT_CNT`"]
pub type EFUSE_RD_FLASH_CRYPT_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_RD_FLASH_CRYPT_CNT`"]
pub struct EFUSE_RD_FLASH_CRYPT_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_FLASH_CRYPT_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x7f << 20)) | (((value as u32) & 0x7f) << 20);
        self.w
    }
}
#[doc = "Reader of field `EFUSE_RD_EFUSE_RD_DIS`"]
pub type EFUSE_RD_EFUSE_RD_DIS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `EFUSE_RD_EFUSE_RD_DIS`"]
pub struct EFUSE_RD_EFUSE_RD_DIS_W<'a> {
    w: &'a mut W,
}
impl<'a> EFUSE_RD_EFUSE_RD_DIS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 16)) | (((value as u32) & 0x0f) << 16);
        self.w
    }
}
impl R {
    #[doc = "Bits 20:26 - read for flash_crypt_cnt"]
    #[inline(always)]
    pub fn efuse_rd_flash_crypt_cnt(&self) -> EFUSE_RD_FLASH_CRYPT_CNT_R {
        EFUSE_RD_FLASH_CRYPT_CNT_R::new(((self.bits >> 20) & 0x7f) as u8)
    }
    #[doc = "Bits 16:19 - read for efuse_rd_disable"]
    #[inline(always)]
    pub fn efuse_rd_efuse_rd_dis(&self) -> EFUSE_RD_EFUSE_RD_DIS_R {
        EFUSE_RD_EFUSE_RD_DIS_R::new(((self.bits >> 16) & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 20:26 - read for flash_crypt_cnt"]
    #[inline(always)]
    pub fn efuse_rd_flash_crypt_cnt(&mut self) -> EFUSE_RD_FLASH_CRYPT_CNT_W {
        EFUSE_RD_FLASH_CRYPT_CNT_W { w: self }
    }
    #[doc = "Bits 16:19 - read for efuse_rd_disable"]
    #[inline(always)]
    pub fn efuse_rd_efuse_rd_dis(&mut self) -> EFUSE_RD_EFUSE_RD_DIS_W {
        EFUSE_RD_EFUSE_RD_DIS_W { w: self }
    }
}
