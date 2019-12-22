#[doc = "Reader of register SLC_SDIO_CRC_ST0_REG"]
pub type R = crate::R<u32, super::SLC_SDIO_CRC_ST0_REG>;
#[doc = "Writer for register SLC_SDIO_CRC_ST0_REG"]
pub type W = crate::W<u32, super::SLC_SDIO_CRC_ST0_REG>;
#[doc = "Register SLC_SDIO_CRC_ST0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SLC_SDIO_CRC_ST0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_DAT3_CRC_ERR_CNT`"]
pub type SLC_DAT3_CRC_ERR_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLC_DAT3_CRC_ERR_CNT`"]
pub struct SLC_DAT3_CRC_ERR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_DAT3_CRC_ERR_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 24)) | (((value as u32) & 0xff) << 24);
        self.w
    }
}
#[doc = "Reader of field `SLC_DAT2_CRC_ERR_CNT`"]
pub type SLC_DAT2_CRC_ERR_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLC_DAT2_CRC_ERR_CNT`"]
pub struct SLC_DAT2_CRC_ERR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_DAT2_CRC_ERR_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 16)) | (((value as u32) & 0xff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SLC_DAT1_CRC_ERR_CNT`"]
pub type SLC_DAT1_CRC_ERR_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLC_DAT1_CRC_ERR_CNT`"]
pub struct SLC_DAT1_CRC_ERR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_DAT1_CRC_ERR_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `SLC_DAT0_CRC_ERR_CNT`"]
pub type SLC_DAT0_CRC_ERR_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLC_DAT0_CRC_ERR_CNT`"]
pub struct SLC_DAT0_CRC_ERR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_DAT0_CRC_ERR_CNT_W<'a> {
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
    pub fn slc_dat3_crc_err_cnt(&self) -> SLC_DAT3_CRC_ERR_CNT_R {
        SLC_DAT3_CRC_ERR_CNT_R::new(((self.bits >> 24) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn slc_dat2_crc_err_cnt(&self) -> SLC_DAT2_CRC_ERR_CNT_R {
        SLC_DAT2_CRC_ERR_CNT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn slc_dat1_crc_err_cnt(&self) -> SLC_DAT1_CRC_ERR_CNT_R {
        SLC_DAT1_CRC_ERR_CNT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn slc_dat0_crc_err_cnt(&self) -> SLC_DAT0_CRC_ERR_CNT_R {
        SLC_DAT0_CRC_ERR_CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn slc_dat3_crc_err_cnt(&mut self) -> SLC_DAT3_CRC_ERR_CNT_W {
        SLC_DAT3_CRC_ERR_CNT_W { w: self }
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn slc_dat2_crc_err_cnt(&mut self) -> SLC_DAT2_CRC_ERR_CNT_W {
        SLC_DAT2_CRC_ERR_CNT_W { w: self }
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn slc_dat1_crc_err_cnt(&mut self) -> SLC_DAT1_CRC_ERR_CNT_W {
        SLC_DAT1_CRC_ERR_CNT_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn slc_dat0_crc_err_cnt(&mut self) -> SLC_DAT0_CRC_ERR_CNT_W {
        SLC_DAT0_CRC_ERR_CNT_W { w: self }
    }
}
