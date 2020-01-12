#[doc = "Reader of register BROWN_OUT"]
pub type R = crate::R<u32, super::BROWN_OUT>;
#[doc = "Writer for register BROWN_OUT"]
pub type W = crate::W<u32, super::BROWN_OUT>;
#[doc = "Register BROWN_OUT `reset()`'s with value 0"]
impl crate::ResetValue for super::BROWN_OUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `BROWN_OUT_DET`"]
pub type BROWN_OUT_DET_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BROWN_OUT_DET`"]
pub struct BROWN_OUT_DET_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_DET_W<'a> {
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
#[doc = "Reader of field `BROWN_OUT_ENA`"]
pub type BROWN_OUT_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BROWN_OUT_ENA`"]
pub struct BROWN_OUT_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `DBROWN_OUT_THRES`"]
pub type DBROWN_OUT_THRES_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DBROWN_OUT_THRES`"]
pub struct DBROWN_OUT_THRES_W<'a> {
    w: &'a mut W,
}
impl<'a> DBROWN_OUT_THRES_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 27)) | (((value as u32) & 0x07) << 27);
        self.w
    }
}
#[doc = "Reader of field `BROWN_OUT_RST_ENA`"]
pub type BROWN_OUT_RST_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BROWN_OUT_RST_ENA`"]
pub struct BROWN_OUT_RST_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_RST_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 26)) | (((value as u32) & 0x01) << 26);
        self.w
    }
}
#[doc = "Reader of field `BROWN_OUT_RST_WAIT`"]
pub type BROWN_OUT_RST_WAIT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `BROWN_OUT_RST_WAIT`"]
pub struct BROWN_OUT_RST_WAIT_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_RST_WAIT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03ff << 16)) | (((value as u32) & 0x03ff) << 16);
        self.w
    }
}
#[doc = "Reader of field `BROWN_OUT_PD_RF_ENA`"]
pub type BROWN_OUT_PD_RF_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BROWN_OUT_PD_RF_ENA`"]
pub struct BROWN_OUT_PD_RF_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_PD_RF_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `BROWN_OUT_CLOSE_FLASH_ENA`"]
pub type BROWN_OUT_CLOSE_FLASH_ENA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `BROWN_OUT_CLOSE_FLASH_ENA`"]
pub struct BROWN_OUT_CLOSE_FLASH_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> BROWN_OUT_CLOSE_FLASH_ENA_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - brown out detect"]
    #[inline(always)]
    pub fn brown_out_det(&self) -> BROWN_OUT_DET_R {
        BROWN_OUT_DET_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - enable brown out"]
    #[inline(always)]
    pub fn brown_out_ena(&self) -> BROWN_OUT_ENA_R {
        BROWN_OUT_ENA_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bits 27:29 - brown out threshold"]
    #[inline(always)]
    pub fn dbrown_out_thres(&self) -> DBROWN_OUT_THRES_R {
        DBROWN_OUT_THRES_R::new(((self.bits >> 27) & 0x07) as u8)
    }
    #[doc = "Bit 26 - enable brown out reset"]
    #[inline(always)]
    pub fn brown_out_rst_ena(&self) -> BROWN_OUT_RST_ENA_R {
        BROWN_OUT_RST_ENA_R::new(((self.bits >> 26) & 0x01) != 0)
    }
    #[doc = "Bits 16:25 - brown out reset wait cycles"]
    #[inline(always)]
    pub fn brown_out_rst_wait(&self) -> BROWN_OUT_RST_WAIT_R {
        BROWN_OUT_RST_WAIT_R::new(((self.bits >> 16) & 0x03ff) as u16)
    }
    #[doc = "Bit 15 - enable power down RF when brown out happens"]
    #[inline(always)]
    pub fn brown_out_pd_rf_ena(&self) -> BROWN_OUT_PD_RF_ENA_R {
        BROWN_OUT_PD_RF_ENA_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14 - enable close flash when brown out happens"]
    #[inline(always)]
    pub fn brown_out_close_flash_ena(&self) -> BROWN_OUT_CLOSE_FLASH_ENA_R {
        BROWN_OUT_CLOSE_FLASH_ENA_R::new(((self.bits >> 14) & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 31 - brown out detect"]
    #[inline(always)]
    pub fn brown_out_det(&mut self) -> BROWN_OUT_DET_W {
        BROWN_OUT_DET_W { w: self }
    }
    #[doc = "Bit 30 - enable brown out"]
    #[inline(always)]
    pub fn brown_out_ena(&mut self) -> BROWN_OUT_ENA_W {
        BROWN_OUT_ENA_W { w: self }
    }
    #[doc = "Bits 27:29 - brown out threshold"]
    #[inline(always)]
    pub fn dbrown_out_thres(&mut self) -> DBROWN_OUT_THRES_W {
        DBROWN_OUT_THRES_W { w: self }
    }
    #[doc = "Bit 26 - enable brown out reset"]
    #[inline(always)]
    pub fn brown_out_rst_ena(&mut self) -> BROWN_OUT_RST_ENA_W {
        BROWN_OUT_RST_ENA_W { w: self }
    }
    #[doc = "Bits 16:25 - brown out reset wait cycles"]
    #[inline(always)]
    pub fn brown_out_rst_wait(&mut self) -> BROWN_OUT_RST_WAIT_W {
        BROWN_OUT_RST_WAIT_W { w: self }
    }
    #[doc = "Bit 15 - enable power down RF when brown out happens"]
    #[inline(always)]
    pub fn brown_out_pd_rf_ena(&mut self) -> BROWN_OUT_PD_RF_ENA_W {
        BROWN_OUT_PD_RF_ENA_W { w: self }
    }
    #[doc = "Bit 14 - enable close flash when brown out happens"]
    #[inline(always)]
    pub fn brown_out_close_flash_ena(&mut self) -> BROWN_OUT_CLOSE_FLASH_ENA_W {
        BROWN_OUT_CLOSE_FLASH_ENA_W { w: self }
    }
}
