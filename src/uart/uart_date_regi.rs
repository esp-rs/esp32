#[doc = "Reader of register UART_DATE_REG(i)"]
pub type R = crate::R<u32, super::UART_DATE_REGI>;
#[doc = "Writer for register UART_DATE_REG(i)"]
pub type W = crate::W<u32, super::UART_DATE_REGI>;
#[doc = "Register UART_DATE_REG(i) `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_DATE_REGI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_DATE`"]
pub type UART_DATE_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UART_DATE`"]
pub struct UART_DATE_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_DATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn uart_date(&self) -> UART_DATE_R {
        UART_DATE_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn uart_date(&mut self) -> UART_DATE_W {
        UART_DATE_W { w: self }
    }
}
