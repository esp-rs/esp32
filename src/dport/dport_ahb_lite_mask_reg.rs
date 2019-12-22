#[doc = "Reader of register DPORT_AHB_LITE_MASK_REG"]
pub type R = crate::R<u32, super::DPORT_AHB_LITE_MASK_REG>;
#[doc = "Writer for register DPORT_AHB_LITE_MASK_REG"]
pub type W = crate::W<u32, super::DPORT_AHB_LITE_MASK_REG>;
#[doc = "Register DPORT_AHB_LITE_MASK_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_AHB_LITE_MASK_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_AHB_LITE_SDHOST_PID_REG`"]
pub type DPORT_AHB_LITE_SDHOST_PID_REG_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_AHB_LITE_SDHOST_PID_REG`"]
pub struct DPORT_AHB_LITE_SDHOST_PID_REG_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_AHB_LITE_SDHOST_PID_REG_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 11)) | (((value as u32) & 0x07) << 11);
        self.w
    }
}
#[doc = "Reader of field `DPORT_AHB_LITE_MASK_APPDPORT`"]
pub type DPORT_AHB_LITE_MASK_APPDPORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_AHB_LITE_MASK_APPDPORT`"]
pub struct DPORT_AHB_LITE_MASK_APPDPORT_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_AHB_LITE_MASK_APPDPORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 10)) | (((value as u32) & 0x01) << 10);
        self.w
    }
}
#[doc = "Reader of field `DPORT_AHB_LITE_MASK_PRODPORT`"]
pub type DPORT_AHB_LITE_MASK_PRODPORT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_AHB_LITE_MASK_PRODPORT`"]
pub struct DPORT_AHB_LITE_MASK_PRODPORT_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_AHB_LITE_MASK_PRODPORT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 9)) | (((value as u32) & 0x01) << 9);
        self.w
    }
}
#[doc = "Reader of field `DPORT_AHB_LITE_MASK_SDIO`"]
pub type DPORT_AHB_LITE_MASK_SDIO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_AHB_LITE_MASK_SDIO`"]
pub struct DPORT_AHB_LITE_MASK_SDIO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_AHB_LITE_MASK_SDIO_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 8)) | (((value as u32) & 0x01) << 8);
        self.w
    }
}
#[doc = "Reader of field `DPORT_AHB_LITE_MASK_APP`"]
pub type DPORT_AHB_LITE_MASK_APP_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_AHB_LITE_MASK_APP`"]
pub struct DPORT_AHB_LITE_MASK_APP_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_AHB_LITE_MASK_APP_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `DPORT_AHB_LITE_MASK_PRO`"]
pub type DPORT_AHB_LITE_MASK_PRO_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_AHB_LITE_MASK_PRO`"]
pub struct DPORT_AHB_LITE_MASK_PRO_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_AHB_LITE_MASK_PRO_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn dport_ahb_lite_sdhost_pid_reg(&self) -> DPORT_AHB_LITE_SDHOST_PID_REG_R {
        DPORT_AHB_LITE_SDHOST_PID_REG_R::new(((self.bits >> 11) & 0x07) as u8)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dport_ahb_lite_mask_appdport(&self) -> DPORT_AHB_LITE_MASK_APPDPORT_R {
        DPORT_AHB_LITE_MASK_APPDPORT_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dport_ahb_lite_mask_prodport(&self) -> DPORT_AHB_LITE_MASK_PRODPORT_R {
        DPORT_AHB_LITE_MASK_PRODPORT_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dport_ahb_lite_mask_sdio(&self) -> DPORT_AHB_LITE_MASK_SDIO_R {
        DPORT_AHB_LITE_MASK_SDIO_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dport_ahb_lite_mask_app(&self) -> DPORT_AHB_LITE_MASK_APP_R {
        DPORT_AHB_LITE_MASK_APP_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_ahb_lite_mask_pro(&self) -> DPORT_AHB_LITE_MASK_PRO_R {
        DPORT_AHB_LITE_MASK_PRO_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bits 11:13"]
    #[inline(always)]
    pub fn dport_ahb_lite_sdhost_pid_reg(&mut self) -> DPORT_AHB_LITE_SDHOST_PID_REG_W {
        DPORT_AHB_LITE_SDHOST_PID_REG_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn dport_ahb_lite_mask_appdport(&mut self) -> DPORT_AHB_LITE_MASK_APPDPORT_W {
        DPORT_AHB_LITE_MASK_APPDPORT_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn dport_ahb_lite_mask_prodport(&mut self) -> DPORT_AHB_LITE_MASK_PRODPORT_W {
        DPORT_AHB_LITE_MASK_PRODPORT_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn dport_ahb_lite_mask_sdio(&mut self) -> DPORT_AHB_LITE_MASK_SDIO_W {
        DPORT_AHB_LITE_MASK_SDIO_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn dport_ahb_lite_mask_app(&mut self) -> DPORT_AHB_LITE_MASK_APP_W {
        DPORT_AHB_LITE_MASK_APP_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn dport_ahb_lite_mask_pro(&mut self) -> DPORT_AHB_LITE_MASK_PRO_W {
        DPORT_AHB_LITE_MASK_PRO_W { w: self }
    }
}
