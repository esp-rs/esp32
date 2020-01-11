#[doc = "Reader of register SDIO_CRC_ST1"]
pub type R = crate::R<u32, super::SDIO_CRC_ST1>;
#[doc = "Writer for register SDIO_CRC_ST1"]
pub type W = crate::W<u32, super::SDIO_CRC_ST1>;
#[doc = "Register SDIO_CRC_ST1 `reset()`'s with value 0"]
impl crate::ResetValue for super::SDIO_CRC_ST1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_ERR_CNT_CLR`"]
pub type SLC_ERR_CNT_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_ERR_CNT_CLR`"]
pub struct SLC_ERR_CNT_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_ERR_CNT_CLR_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `SLC_CMD_CRC_ERR_CNT`"]
pub type SLC_CMD_CRC_ERR_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLC_CMD_CRC_ERR_CNT`"]
pub struct SLC_CMD_CRC_ERR_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_CMD_CRC_ERR_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slc_err_cnt_clr(&self) -> SLC_ERR_CNT_CLR_R {
        SLC_ERR_CNT_CLR_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn slc_cmd_crc_err_cnt(&self) -> SLC_CMD_CRC_ERR_CNT_R {
        SLC_CMD_CRC_ERR_CNT_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slc_err_cnt_clr(&mut self) -> SLC_ERR_CNT_CLR_W {
        SLC_ERR_CNT_CLR_W { w: self }
    }
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn slc_cmd_crc_err_cnt(&mut self) -> SLC_CMD_CRC_ERR_CNT_W {
        SLC_CMD_CRC_ERR_CNT_W { w: self }
    }
}
