#[doc = "Reader of register GPIO_STATUS1_W1TS_REG"]
pub type R = crate::R<u32, super::GPIO_STATUS1_W1TS_REG>;
#[doc = "Writer for register GPIO_STATUS1_W1TS_REG"]
pub type W = crate::W<u32, super::GPIO_STATUS1_W1TS_REG>;
#[doc = "Register GPIO_STATUS1_W1TS_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::GPIO_STATUS1_W1TS_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_STATUS1_INT_W1TS`"]
pub type GPIO_STATUS1_INT_W1TS_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_STATUS1_INT_W1TS`"]
pub struct GPIO_STATUS1_INT_W1TS_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_STATUS1_INT_W1TS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status write 1 to set"]
    #[inline(always)]
    pub fn gpio_status1_int_w1ts(&self) -> GPIO_STATUS1_INT_W1TS_R {
        GPIO_STATUS1_INT_W1TS_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 interrupt status write 1 to set"]
    #[inline(always)]
    pub fn gpio_status1_int_w1ts(&mut self) -> GPIO_STATUS1_INT_W1TS_W {
        GPIO_STATUS1_INT_W1TS_W { w: self }
    }
}
