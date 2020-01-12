#[doc = "Reader of register HOLD_FORCE"]
pub type R = crate::R<u32, super::HOLD_FORCE>;
#[doc = "Writer for register HOLD_FORCE"]
pub type W = crate::W<u32, super::HOLD_FORCE>;
#[doc = "Register HOLD_FORCE `reset()`'s with value 0"]
impl crate::ResetValue for super::HOLD_FORCE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `X32N_HOLD_FORCE`"]
pub type X32N_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X32N_HOLD_FORCE`"]
pub struct X32N_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32N_HOLD_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `X32P_HOLD_FORCE`"]
pub type X32P_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `X32P_HOLD_FORCE`"]
pub struct X32P_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> X32P_HOLD_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `TOUCH_PAD7_HOLD_FORCE`"]
pub type TOUCH_PAD7_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUCH_PAD7_HOLD_FORCE`"]
pub struct TOUCH_PAD7_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD7_HOLD_FORCE_W<'a> {
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
#[doc = "Reader of field `TOUCH_PAD6_HOLD_FORCE`"]
pub type TOUCH_PAD6_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUCH_PAD6_HOLD_FORCE`"]
pub struct TOUCH_PAD6_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD6_HOLD_FORCE_W<'a> {
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
#[doc = "Reader of field `TOUCH_PAD5_HOLD_FORCE`"]
pub type TOUCH_PAD5_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUCH_PAD5_HOLD_FORCE`"]
pub struct TOUCH_PAD5_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD5_HOLD_FORCE_W<'a> {
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
#[doc = "Reader of field `TOUCH_PAD4_HOLD_FORCE`"]
pub type TOUCH_PAD4_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUCH_PAD4_HOLD_FORCE`"]
pub struct TOUCH_PAD4_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD4_HOLD_FORCE_W<'a> {
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
#[doc = "Reader of field `TOUCH_PAD3_HOLD_FORCE`"]
pub type TOUCH_PAD3_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUCH_PAD3_HOLD_FORCE`"]
pub struct TOUCH_PAD3_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD3_HOLD_FORCE_W<'a> {
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
#[doc = "Reader of field `TOUCH_PAD2_HOLD_FORCE`"]
pub type TOUCH_PAD2_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUCH_PAD2_HOLD_FORCE`"]
pub struct TOUCH_PAD2_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD2_HOLD_FORCE_W<'a> {
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
#[doc = "Reader of field `TOUCH_PAD1_HOLD_FORCE`"]
pub type TOUCH_PAD1_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUCH_PAD1_HOLD_FORCE`"]
pub struct TOUCH_PAD1_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD1_HOLD_FORCE_W<'a> {
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
#[doc = "Reader of field `TOUCH_PAD0_HOLD_FORCE`"]
pub type TOUCH_PAD0_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TOUCH_PAD0_HOLD_FORCE`"]
pub struct TOUCH_PAD0_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> TOUCH_PAD0_HOLD_FORCE_W<'a> {
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
#[doc = "Reader of field `SENSE4_HOLD_FORCE`"]
pub type SENSE4_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENSE4_HOLD_FORCE`"]
pub struct SENSE4_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE4_HOLD_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `SENSE3_HOLD_FORCE`"]
pub type SENSE3_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENSE3_HOLD_FORCE`"]
pub struct SENSE3_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE3_HOLD_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `SENSE2_HOLD_FORCE`"]
pub type SENSE2_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENSE2_HOLD_FORCE`"]
pub struct SENSE2_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE2_HOLD_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `SENSE1_HOLD_FORCE`"]
pub type SENSE1_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SENSE1_HOLD_FORCE`"]
pub struct SENSE1_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> SENSE1_HOLD_FORCE_W<'a> {
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
#[doc = "Reader of field `PDAC2_HOLD_FORCE`"]
pub type PDAC2_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDAC2_HOLD_FORCE`"]
pub struct PDAC2_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC2_HOLD_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `PDAC1_HOLD_FORCE`"]
pub type PDAC1_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PDAC1_HOLD_FORCE`"]
pub struct PDAC1_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> PDAC1_HOLD_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `ADC2_HOLD_FORCE`"]
pub type ADC2_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC2_HOLD_FORCE`"]
pub struct ADC2_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC2_HOLD_FORCE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `ADC1_HOLD_FORCE`"]
pub type ADC1_HOLD_FORCE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `ADC1_HOLD_FORCE`"]
pub struct ADC1_HOLD_FORCE_W<'a> {
    w: &'a mut W,
}
impl<'a> ADC1_HOLD_FORCE_W<'a> {
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
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn x32n_hold_force(&self) -> X32N_HOLD_FORCE_R {
        X32N_HOLD_FORCE_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn x32p_hold_force(&self) -> X32P_HOLD_FORCE_R {
        X32P_HOLD_FORCE_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn touch_pad7_hold_force(&self) -> TOUCH_PAD7_HOLD_FORCE_R {
        TOUCH_PAD7_HOLD_FORCE_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn touch_pad6_hold_force(&self) -> TOUCH_PAD6_HOLD_FORCE_R {
        TOUCH_PAD6_HOLD_FORCE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn touch_pad5_hold_force(&self) -> TOUCH_PAD5_HOLD_FORCE_R {
        TOUCH_PAD5_HOLD_FORCE_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_pad4_hold_force(&self) -> TOUCH_PAD4_HOLD_FORCE_R {
        TOUCH_PAD4_HOLD_FORCE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn touch_pad3_hold_force(&self) -> TOUCH_PAD3_HOLD_FORCE_R {
        TOUCH_PAD3_HOLD_FORCE_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn touch_pad2_hold_force(&self) -> TOUCH_PAD2_HOLD_FORCE_R {
        TOUCH_PAD2_HOLD_FORCE_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn touch_pad1_hold_force(&self) -> TOUCH_PAD1_HOLD_FORCE_R {
        TOUCH_PAD1_HOLD_FORCE_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn touch_pad0_hold_force(&self) -> TOUCH_PAD0_HOLD_FORCE_R {
        TOUCH_PAD0_HOLD_FORCE_R::new(((self.bits >> 8) & 0x01) != 0)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sense4_hold_force(&self) -> SENSE4_HOLD_FORCE_R {
        SENSE4_HOLD_FORCE_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sense3_hold_force(&self) -> SENSE3_HOLD_FORCE_R {
        SENSE3_HOLD_FORCE_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sense2_hold_force(&self) -> SENSE2_HOLD_FORCE_R {
        SENSE2_HOLD_FORCE_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sense1_hold_force(&self) -> SENSE1_HOLD_FORCE_R {
        SENSE1_HOLD_FORCE_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pdac2_hold_force(&self) -> PDAC2_HOLD_FORCE_R {
        PDAC2_HOLD_FORCE_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pdac1_hold_force(&self) -> PDAC1_HOLD_FORCE_R {
        PDAC1_HOLD_FORCE_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc2_hold_force(&self) -> ADC2_HOLD_FORCE_R {
        ADC2_HOLD_FORCE_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc1_hold_force(&self) -> ADC1_HOLD_FORCE_R {
        ADC1_HOLD_FORCE_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn x32n_hold_force(&mut self) -> X32N_HOLD_FORCE_W {
        X32N_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn x32p_hold_force(&mut self) -> X32P_HOLD_FORCE_W {
        X32P_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn touch_pad7_hold_force(&mut self) -> TOUCH_PAD7_HOLD_FORCE_W {
        TOUCH_PAD7_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn touch_pad6_hold_force(&mut self) -> TOUCH_PAD6_HOLD_FORCE_W {
        TOUCH_PAD6_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn touch_pad5_hold_force(&mut self) -> TOUCH_PAD5_HOLD_FORCE_W {
        TOUCH_PAD5_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn touch_pad4_hold_force(&mut self) -> TOUCH_PAD4_HOLD_FORCE_W {
        TOUCH_PAD4_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn touch_pad3_hold_force(&mut self) -> TOUCH_PAD3_HOLD_FORCE_W {
        TOUCH_PAD3_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn touch_pad2_hold_force(&mut self) -> TOUCH_PAD2_HOLD_FORCE_W {
        TOUCH_PAD2_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn touch_pad1_hold_force(&mut self) -> TOUCH_PAD1_HOLD_FORCE_W {
        TOUCH_PAD1_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn touch_pad0_hold_force(&mut self) -> TOUCH_PAD0_HOLD_FORCE_W {
        TOUCH_PAD0_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn sense4_hold_force(&mut self) -> SENSE4_HOLD_FORCE_W {
        SENSE4_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn sense3_hold_force(&mut self) -> SENSE3_HOLD_FORCE_W {
        SENSE3_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn sense2_hold_force(&mut self) -> SENSE2_HOLD_FORCE_W {
        SENSE2_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn sense1_hold_force(&mut self) -> SENSE1_HOLD_FORCE_W {
        SENSE1_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn pdac2_hold_force(&mut self) -> PDAC2_HOLD_FORCE_W {
        PDAC2_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn pdac1_hold_force(&mut self) -> PDAC1_HOLD_FORCE_W {
        PDAC1_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn adc2_hold_force(&mut self) -> ADC2_HOLD_FORCE_W {
        ADC2_HOLD_FORCE_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn adc1_hold_force(&mut self) -> ADC1_HOLD_FORCE_W {
        ADC1_HOLD_FORCE_W { w: self }
    }
}
