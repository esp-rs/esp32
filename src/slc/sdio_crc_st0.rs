#[doc = "Reader of register SDIO_CRC_ST0"]
pub type R = crate::R<u32, super::SDIO_CRC_ST0>;
#[doc = "Writer for register SDIO_CRC_ST0"]
pub type W = crate::W<u32, super::SDIO_CRC_ST0>;
#[doc = "Register SDIO_CRC_ST0 `reset()`'s with value 0"]
impl crate::ResetValue for super::SDIO_CRC_ST0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DAT3_CRC_ERR_CNT`"]
pub type DAT3_CRC_ERR_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAT3_CRC_ERR_CNT`"]
pub struct DAT3_CRC_ERR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT3_CRC_ERR_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `DAT2_CRC_ERR_CNT`"]
pub type DAT2_CRC_ERR_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAT2_CRC_ERR_CNT`"]
pub struct DAT2_CRC_ERR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT2_CRC_ERR_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `DAT1_CRC_ERR_CNT`"]
pub type DAT1_CRC_ERR_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAT1_CRC_ERR_CNT`"]
pub struct DAT1_CRC_ERR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT1_CRC_ERR_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `DAT0_CRC_ERR_CNT`"]
pub type DAT0_CRC_ERR_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DAT0_CRC_ERR_CNT`"]
pub struct DAT0_CRC_ERR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> DAT0_CRC_ERR_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn dat3_crc_err_cnt(&self) -> DAT3_CRC_ERR_CNT_R {
        DAT3_CRC_ERR_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn dat2_crc_err_cnt(&self) -> DAT2_CRC_ERR_CNT_R {
        DAT2_CRC_ERR_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn dat1_crc_err_cnt(&self) -> DAT1_CRC_ERR_CNT_R {
        DAT1_CRC_ERR_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dat0_crc_err_cnt(&self) -> DAT0_CRC_ERR_CNT_R {
        DAT0_CRC_ERR_CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn dat3_crc_err_cnt(&mut self) -> DAT3_CRC_ERR_CNT_W {
        DAT3_CRC_ERR_CNT_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn dat2_crc_err_cnt(&mut self) -> DAT2_CRC_ERR_CNT_W {
        DAT2_CRC_ERR_CNT_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn dat1_crc_err_cnt(&mut self) -> DAT1_CRC_ERR_CNT_W {
        DAT1_CRC_ERR_CNT_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn dat0_crc_err_cnt(&mut self) -> DAT0_CRC_ERR_CNT_W {
        DAT0_CRC_ERR_CNT_W { w: self }
    }
}
