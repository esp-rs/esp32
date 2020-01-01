#[doc = "Reader of register MEM_TX_STATUS"]
pub type R = crate::R<u32, super::MEM_TX_STATUS>;
#[doc = "Writer for register MEM_TX_STATUS"]
pub type W = crate::W<u32, super::MEM_TX_STATUS>;
#[doc = "Register MEM_TX_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_TX_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_MEM_TX_STATUS`"]
pub type UART_MEM_TX_STATUS_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UART_MEM_TX_STATUS`"]
pub struct UART_MEM_TX_STATUS_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_MEM_TX_STATUS_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn uart_mem_tx_status(&self) -> UART_MEM_TX_STATUS_R {
        UART_MEM_TX_STATUS_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23"]
    #[inline(always)]
    pub fn uart_mem_tx_status(&mut self) -> UART_MEM_TX_STATUS_W {
        UART_MEM_TX_STATUS_W { w: self }
    }
}
