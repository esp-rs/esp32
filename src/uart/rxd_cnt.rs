#[doc = "Reader of register RXD_CNT"]
pub type R = crate::R<u32, super::RXD_CNT>;
#[doc = "Writer for register RXD_CNT"]
pub type W = crate::W<u32, super::RXD_CNT>;
#[doc = "Register RXD_CNT `reset()`'s with value 0"]
impl crate::ResetValue for super::RXD_CNT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_RXD_EDGE_CNT`"]
pub type UART_RXD_EDGE_CNT_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `UART_RXD_EDGE_CNT`"]
pub struct UART_RXD_EDGE_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_RXD_EDGE_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03ff) | ((value as u32) & 0x03ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:9 - This register stores the count of rxd edge change. it is used in baudrate-detect process."]
    #[inline(always)]
    pub fn uart_rxd_edge_cnt(&self) -> UART_RXD_EDGE_CNT_R {
        UART_RXD_EDGE_CNT_R::new((self.bits & 0x03ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:9 - This register stores the count of rxd edge change. it is used in baudrate-detect process."]
    #[inline(always)]
    pub fn uart_rxd_edge_cnt(&mut self) -> UART_RXD_EDGE_CNT_W {
        UART_RXD_EDGE_CNT_W { w: self }
    }
}
