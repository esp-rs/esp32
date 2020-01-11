#[doc = "Reader of register ENABLE1"]
pub type R = crate::R<u32, super::ENABLE1>;
#[doc = "Writer for register ENABLE1"]
pub type W = crate::W<u32, super::ENABLE1>;
#[doc = "Register ENABLE1 `reset()`'s with value 0"]
impl crate::ResetValue for super::ENABLE1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `GPIO_ENABLE1_DATA`"]
pub type GPIO_ENABLE1_DATA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `GPIO_ENABLE1_DATA`"]
pub struct GPIO_ENABLE1_DATA_W<'a> {
    w: &'a mut W,
}
impl<'a> GPIO_ENABLE1_DATA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:7 - GPIO32~39 output enable"]
    #[inline(always)]
    pub fn gpio_enable1_data(&self) -> GPIO_ENABLE1_DATA_R {
        GPIO_ENABLE1_DATA_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bits 0:7 - GPIO32~39 output enable"]
    #[inline(always)]
    pub fn gpio_enable1_data(&mut self) -> GPIO_ENABLE1_DATA_W {
        GPIO_ENABLE1_DATA_W { w: self }
    }
}
