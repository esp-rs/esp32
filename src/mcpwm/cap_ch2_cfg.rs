#[doc = "Reader of register CAP_CH2_CFG"]
pub type R = crate::R<u32, super::CAP_CH2_CFG>;
#[doc = "Writer for register CAP_CH2_CFG"]
pub type W = crate::W<u32, super::CAP_CH2_CFG>;
#[doc = "Register CAP_CH2_CFG `reset()`'s with value 0"]
impl crate::ResetValue for super::CAP_CH2_CFG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CAP2_SW`"]
pub type CAP2_SW_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP2_SW`"]
pub struct CAP2_SW_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2_SW_W<'a> {
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
#[doc = "Reader of field `CAP2_IN_INVERT`"]
pub type CAP2_IN_INVERT_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP2_IN_INVERT`"]
pub struct CAP2_IN_INVERT_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2_IN_INVERT_W<'a> {
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
#[doc = "Reader of field `CAP2_PRESCALE`"]
pub type CAP2_PRESCALE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAP2_PRESCALE`"]
pub struct CAP2_PRESCALE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2_PRESCALE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 3)) | (((value as u32) & 0xff) << 3);
        self.w
    }
}
#[doc = "Reader of field `CAP2_MODE`"]
pub type CAP2_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `CAP2_MODE`"]
pub struct CAP2_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `CAP2_EN`"]
pub type CAP2_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CAP2_EN`"]
pub struct CAP2_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CAP2_EN_W<'a> {
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
    #[doc = "Bit 12 - Write 1 will trigger a software forced capture on channel 2"]
    #[inline(always)]
    pub fn cap2_sw(&self) -> CAP2_SW_R {
        CAP2_SW_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11 - When set CAP2 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    pub fn cap2_in_invert(&self) -> CAP2_IN_INVERT_R {
        CAP2_IN_INVERT_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bits 3:10 - Value of prescale on possitive edge of CAP2. Prescale value = PWM_CAP2_PRESCALE + 1"]
    #[inline(always)]
    pub fn cap2_prescale(&self) -> CAP2_PRESCALE_R {
        CAP2_PRESCALE_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 2 after prescale. bit0: negedge cap en bit1: posedge cap en"]
    #[inline(always)]
    pub fn cap2_mode(&self) -> CAP2_MODE_R {
        CAP2_MODE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - When set capture on channel 2 is enabled"]
    #[inline(always)]
    pub fn cap2_en(&self) -> CAP2_EN_R {
        CAP2_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 12 - Write 1 will trigger a software forced capture on channel 2"]
    #[inline(always)]
    pub fn cap2_sw(&mut self) -> CAP2_SW_W {
        CAP2_SW_W { w: self }
    }
    #[doc = "Bit 11 - When set CAP2 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    pub fn cap2_in_invert(&mut self) -> CAP2_IN_INVERT_W {
        CAP2_IN_INVERT_W { w: self }
    }
    #[doc = "Bits 3:10 - Value of prescale on possitive edge of CAP2. Prescale value = PWM_CAP2_PRESCALE + 1"]
    #[inline(always)]
    pub fn cap2_prescale(&mut self) -> CAP2_PRESCALE_W {
        CAP2_PRESCALE_W { w: self }
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 2 after prescale. bit0: negedge cap en bit1: posedge cap en"]
    #[inline(always)]
    pub fn cap2_mode(&mut self) -> CAP2_MODE_W {
        CAP2_MODE_W { w: self }
    }
    #[doc = "Bit 0 - When set capture on channel 2 is enabled"]
    #[inline(always)]
    pub fn cap2_en(&mut self) -> CAP2_EN_W {
        CAP2_EN_W { w: self }
    }
}
