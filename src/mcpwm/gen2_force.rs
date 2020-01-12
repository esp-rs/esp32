#[doc = "Reader of register GEN2_FORCE"]
pub type R = crate::R<u32, super::GEN2_FORCE>;
#[doc = "Writer for register GEN2_FORCE"]
pub type W = crate::W<u32, super::GEN2_FORCE>;
#[doc = "Register GEN2_FORCE `reset()`'s with value 0"]
impl crate::ResetValue for super::GEN2_FORCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GEN2_B_NCIFORCE_MODE`"]
pub type GEN2_B_NCIFORCE_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GEN2_B_NCIFORCE_MODE`"]
pub struct GEN2_B_NCIFORCE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN2_B_NCIFORCE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 14)) | (((value as u32) & 0x03) << 14);
        self.w
    }
}
#[doc = "Reader of field `GEN2_B_NCIFORCE`"]
pub type GEN2_B_NCIFORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEN2_B_NCIFORCE`"]
pub struct GEN2_B_NCIFORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN2_B_NCIFORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `GEN2_A_NCIFORCE_MODE`"]
pub type GEN2_A_NCIFORCE_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GEN2_A_NCIFORCE_MODE`"]
pub struct GEN2_A_NCIFORCE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN2_A_NCIFORCE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 11)) | (((value as u32) & 0x03) << 11);
        self.w
    }
}
#[doc = "Reader of field `GEN2_A_NCIFORCE`"]
pub type GEN2_A_NCIFORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GEN2_A_NCIFORCE`"]
pub struct GEN2_A_NCIFORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN2_A_NCIFORCE_W<'a> {
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
#[doc = "Reader of field `GEN2_B_CNTUFORCE_MODE`"]
pub type GEN2_B_CNTUFORCE_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GEN2_B_CNTUFORCE_MODE`"]
pub struct GEN2_B_CNTUFORCE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN2_B_CNTUFORCE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 8)) | (((value as u32) & 0x03) << 8);
        self.w
    }
}
#[doc = "Reader of field `GEN2_A_CNTUFORCE_MODE`"]
pub type GEN2_A_CNTUFORCE_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GEN2_A_CNTUFORCE_MODE`"]
pub struct GEN2_A_CNTUFORCE_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN2_A_CNTUFORCE_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 6)) | (((value as u32) & 0x03) << 6);
        self.w
    }
}
#[doc = "Reader of field `GEN2_CNTUFORCE_UPMETHOD`"]
pub type GEN2_CNTUFORCE_UPMETHOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GEN2_CNTUFORCE_UPMETHOD`"]
pub struct GEN2_CNTUFORCE_UPMETHOD_W<'a> {
    w: &'a mut W,
}
impl<'a> GEN2_CNTUFORCE_UPMETHOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 14:15 - Non-continuous immediate software force mode for PWM2B 0: disabled 1: low 2: high 3: disabled"]
    #[inline(always)]
    pub fn gen2_b_nciforce_mode(&self) -> GEN2_B_NCIFORCE_MODE_R {
        GEN2_B_NCIFORCE_MODE_R::new(((self.bits >> 14) & 0x03) as u8)
    }
    #[doc = "Bit 13 - Non-continuous immediate software force trigger for PWM2B a toggle will trigger a force event"]
    #[inline(always)]
    pub fn gen2_b_nciforce(&self) -> GEN2_B_NCIFORCE_R {
        GEN2_B_NCIFORCE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bits 11:12 - Non-continuous immediate software force mode for PWM2A 0: disabled 1: low 2: high 3: disabled"]
    #[inline(always)]
    pub fn gen2_a_nciforce_mode(&self) -> GEN2_A_NCIFORCE_MODE_R {
        GEN2_A_NCIFORCE_MODE_R::new(((self.bits >> 11) & 0x03) as u8)
    }
    #[doc = "Bit 10 - Non-continuous immediate software force trigger for PWM2A a toggle will trigger a force event"]
    #[inline(always)]
    pub fn gen2_a_nciforce(&self) -> GEN2_A_NCIFORCE_R {
        GEN2_A_NCIFORCE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bits 8:9 - Continuous software force mode for PWM2B. 0: disabled 1: low 2: high 3: disabled"]
    #[inline(always)]
    pub fn gen2_b_cntuforce_mode(&self) -> GEN2_B_CNTUFORCE_MODE_R {
        GEN2_B_CNTUFORCE_MODE_R::new(((self.bits >> 8) & 0x03) as u8)
    }
    #[doc = "Bits 6:7 - Continuous software force mode for PWM2A. 0: disabled 1: low 2: high 3: disabled"]
    #[inline(always)]
    pub fn gen2_a_cntuforce_mode(&self) -> GEN2_A_CNTUFORCE_MODE_R {
        GEN2_A_CNTUFORCE_MODE_R::new(((self.bits >> 6) & 0x03) as u8)
    }
    #[doc = "Bits 0:5 - Update method for continuous software force of PWM generator2. 0: immediate bit0: TEZ bit1: TEP bit2: TEA bit3: TEB bit4: sync bit5: disable update. (TEA/B here and below means an event generated when timer value equals A/B register)"]
    #[inline(always)]
    pub fn gen2_cntuforce_upmethod(&self) -> GEN2_CNTUFORCE_UPMETHOD_R {
        GEN2_CNTUFORCE_UPMETHOD_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 14:15 - Non-continuous immediate software force mode for PWM2B 0: disabled 1: low 2: high 3: disabled"]
    #[inline(always)]
    pub fn gen2_b_nciforce_mode(&mut self) -> GEN2_B_NCIFORCE_MODE_W {
        GEN2_B_NCIFORCE_MODE_W { w: self }
    }
    #[doc = "Bit 13 - Non-continuous immediate software force trigger for PWM2B a toggle will trigger a force event"]
    #[inline(always)]
    pub fn gen2_b_nciforce(&mut self) -> GEN2_B_NCIFORCE_W {
        GEN2_B_NCIFORCE_W { w: self }
    }
    #[doc = "Bits 11:12 - Non-continuous immediate software force mode for PWM2A 0: disabled 1: low 2: high 3: disabled"]
    #[inline(always)]
    pub fn gen2_a_nciforce_mode(&mut self) -> GEN2_A_NCIFORCE_MODE_W {
        GEN2_A_NCIFORCE_MODE_W { w: self }
    }
    #[doc = "Bit 10 - Non-continuous immediate software force trigger for PWM2A a toggle will trigger a force event"]
    #[inline(always)]
    pub fn gen2_a_nciforce(&mut self) -> GEN2_A_NCIFORCE_W {
        GEN2_A_NCIFORCE_W { w: self }
    }
    #[doc = "Bits 8:9 - Continuous software force mode for PWM2B. 0: disabled 1: low 2: high 3: disabled"]
    #[inline(always)]
    pub fn gen2_b_cntuforce_mode(&mut self) -> GEN2_B_CNTUFORCE_MODE_W {
        GEN2_B_CNTUFORCE_MODE_W { w: self }
    }
    #[doc = "Bits 6:7 - Continuous software force mode for PWM2A. 0: disabled 1: low 2: high 3: disabled"]
    #[inline(always)]
    pub fn gen2_a_cntuforce_mode(&mut self) -> GEN2_A_CNTUFORCE_MODE_W {
        GEN2_A_CNTUFORCE_MODE_W { w: self }
    }
    #[doc = "Bits 0:5 - Update method for continuous software force of PWM generator2. 0: immediate bit0: TEZ bit1: TEP bit2: TEA bit3: TEB bit4: sync bit5: disable update. (TEA/B here and below means an event generated when timer value equals A/B register)"]
    #[inline(always)]
    pub fn gen2_cntuforce_upmethod(&mut self) -> GEN2_CNTUFORCE_UPMETHOD_W {
        GEN2_CNTUFORCE_UPMETHOD_W { w: self }
    }
}
