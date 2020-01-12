#[doc = "Reader of register CAP_CH1_CFG"]
pub type R = crate::R<u32, super::CAP_CH1_CFG>;
#[doc = "Writer for register CAP_CH1_CFG"]
pub type W = crate::W<u32, super::CAP_CH1_CFG>;
#[doc = "Register CAP_CH1_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CAP_CH1_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAP1_SW`"]
pub type CAP1_SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP1_SW`"]
pub struct CAP1_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_SW_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `CAP1_IN_INVERT`"]
pub type CAP1_IN_INVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP1_IN_INVERT`"]
pub struct CAP1_IN_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_IN_INVERT_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 11)) | (((value as u32) & 0x01) << 11);
        self.w
    }
}
#[doc = "Reader of field `CAP1_PRESCALE`"]
pub type CAP1_PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAP1_PRESCALE`"]
pub struct CAP1_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 3)) | (((value as u32) & 0xff) << 3);
        self.w
    }
}
#[doc = "Reader of field `CAP1_MODE`"]
pub type CAP1_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAP1_MODE`"]
pub struct CAP1_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `CAP1_EN`"]
pub type CAP1_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP1_EN`"]
pub struct CAP1_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP1_EN_W<'a> {
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
    #[doc = "Bit 12 - Write 1 will trigger a software forced capture on channel 1"]
    #[inline(always)]
    pub fn cap1_sw(&self) -> CAP1_SW_R {
        CAP1_SW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - When set CAP1 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    pub fn cap1_in_invert(&self) -> CAP1_IN_INVERT_R {
        CAP1_IN_INVERT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 3:10 - Value of prescale on possitive edge of CAP1. Prescale value = PWM_CAP1_PRESCALE + 1"]
    #[inline(always)]
    pub fn cap1_prescale(&self) -> CAP1_PRESCALE_R {
        CAP1_PRESCALE_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 1 after prescale. bit0: negedge cap en bit1: posedge cap en"]
    #[inline(always)]
    pub fn cap1_mode(&self) -> CAP1_MODE_R {
        CAP1_MODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - When set capture on channel 1 is enabled"]
    #[inline(always)]
    pub fn cap1_en(&self) -> CAP1_EN_R {
        CAP1_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Write 1 will trigger a software forced capture on channel 1"]
    #[inline(always)]
    pub fn cap1_sw(&mut self) -> CAP1_SW_W {
        CAP1_SW_W { w: self }
    }
    #[doc = "Bit 11 - When set CAP1 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    pub fn cap1_in_invert(&mut self) -> CAP1_IN_INVERT_W {
        CAP1_IN_INVERT_W { w: self }
    }
    #[doc = "Bits 3:10 - Value of prescale on possitive edge of CAP1. Prescale value = PWM_CAP1_PRESCALE + 1"]
    #[inline(always)]
    pub fn cap1_prescale(&mut self) -> CAP1_PRESCALE_W {
        CAP1_PRESCALE_W { w: self }
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 1 after prescale. bit0: negedge cap en bit1: posedge cap en"]
    #[inline(always)]
    pub fn cap1_mode(&mut self) -> CAP1_MODE_W {
        CAP1_MODE_W { w: self }
    }
    #[doc = "Bit 0 - When set capture on channel 1 is enabled"]
    #[inline(always)]
    pub fn cap1_en(&mut self) -> CAP1_EN_W {
        CAP1_EN_W { w: self }
    }
}
