#[doc = "Reader of register GPIO_FUNC29_OUT_SEL_CFG_REG"]
pub type R = crate::R<u32, super::GPIO_FUNC29_OUT_SEL_CFG_REG>;
#[doc = "Writer for register GPIO_FUNC29_OUT_SEL_CFG_REG"]
pub type W = crate::W<u32, super::GPIO_FUNC29_OUT_SEL_CFG_REG>;
#[doc = "Register GPIO_FUNC29_OUT_SEL_CFG_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_FUNC29_OUT_SEL_CFG_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_FUNC29_OEN_INV_SEL`"]
pub type GPIO_FUNC29_OEN_INV_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_FUNC29_OEN_INV_SEL`"]
pub struct GPIO_FUNC29_OEN_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_FUNC29_OEN_INV_SEL_W<'a> {
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
#[doc = "Reader of field `GPIO_FUNC29_OEN_SEL`"]
pub type GPIO_FUNC29_OEN_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_FUNC29_OEN_SEL`"]
pub struct GPIO_FUNC29_OEN_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_FUNC29_OEN_SEL_W<'a> {
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
#[doc = "Reader of field `GPIO_FUNC29_OUT_INV_SEL`"]
pub type GPIO_FUNC29_OUT_INV_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `GPIO_FUNC29_OUT_INV_SEL`"]
pub struct GPIO_FUNC29_OUT_INV_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_FUNC29_OUT_INV_SEL_W<'a> {
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
#[doc = "Reader of field `GPIO_FUNC29_OUT_SEL`"]
pub type GPIO_FUNC29_OUT_SEL_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `GPIO_FUNC29_OUT_SEL`"]
pub struct GPIO_FUNC29_OUT_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_FUNC29_OUT_SEL_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bit 11 - invert the output enable value if you want to revert the output enable value setting the value to 1"]
    #[inline(always)]
    pub fn gpio_func29_oen_inv_sel(&self) -> GPIO_FUNC29_OEN_INV_SEL_R {
        GPIO_FUNC29_OEN_INV_SEL_R::new(((self.bits >> 11) & 0x01) != 0)
    }
    #[doc = "Bit 10 - weather using the logical oen signal or not using the value setting by the register"]
    #[inline(always)]
    pub fn gpio_func29_oen_sel(&self) -> GPIO_FUNC29_OEN_SEL_R {
        GPIO_FUNC29_OEN_SEL_R::new(((self.bits >> 10) & 0x01) != 0)
    }
    #[doc = "Bit 9 - invert the output value if you want to revert the output value setting the value to 1"]
    #[inline(always)]
    pub fn gpio_func29_out_inv_sel(&self) -> GPIO_FUNC29_OUT_INV_SEL_R {
        GPIO_FUNC29_OUT_INV_SEL_R::new(((self.bits >> 9) & 0x01) != 0)
    }
    #[doc = "Bits 0:8 - select one of the 256 output to 40 GPIO"]
    #[inline(always)]
    pub fn gpio_func29_out_sel(&self) -> GPIO_FUNC29_OUT_SEL_R {
        GPIO_FUNC29_OUT_SEL_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bit 11 - invert the output enable value if you want to revert the output enable value setting the value to 1"]
    #[inline(always)]
    pub fn gpio_func29_oen_inv_sel(&mut self) -> GPIO_FUNC29_OEN_INV_SEL_W {
        GPIO_FUNC29_OEN_INV_SEL_W { w: self }
    }
    #[doc = "Bit 10 - weather using the logical oen signal or not using the value setting by the register"]
    #[inline(always)]
    pub fn gpio_func29_oen_sel(&mut self) -> GPIO_FUNC29_OEN_SEL_W {
        GPIO_FUNC29_OEN_SEL_W { w: self }
    }
    #[doc = "Bit 9 - invert the output value if you want to revert the output value setting the value to 1"]
    #[inline(always)]
    pub fn gpio_func29_out_inv_sel(&mut self) -> GPIO_FUNC29_OUT_INV_SEL_W {
        GPIO_FUNC29_OUT_INV_SEL_W { w: self }
    }
    #[doc = "Bits 0:8 - select one of the 256 output to 40 GPIO"]
    #[inline(always)]
    pub fn gpio_func29_out_sel(&mut self) -> GPIO_FUNC29_OUT_SEL_W {
        GPIO_FUNC29_OUT_SEL_W { w: self }
    }
}
