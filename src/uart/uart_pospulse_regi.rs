#[doc = "Reader of register UART_POSPULSE_REG(i)"]
pub type R = crate::R<u32, super::UART_POSPULSE_REGI>;
#[doc = "Writer for register UART_POSPULSE_REG(i)"]
pub type W = crate::W<u32, super::UART_POSPULSE_REGI>;
#[doc = "Register UART_POSPULSE_REG(i) `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_POSPULSE_REGI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_POSEDGE_MIN_CNT`"]
pub type UART_POSEDGE_MIN_CNT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UART_POSEDGE_MIN_CNT`"]
pub struct UART_POSEDGE_MIN_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_POSEDGE_MIN_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19 - This register stores the count of rxd posedge edge. it is used in boudrate-detect process."]
    #[inline(always)]
    pub fn uart_posedge_min_cnt(&self) -> UART_POSEDGE_MIN_CNT_R {
        UART_POSEDGE_MIN_CNT_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19 - This register stores the count of rxd posedge edge. it is used in boudrate-detect process."]
    #[inline(always)]
    pub fn uart_posedge_min_cnt(&mut self) -> UART_POSEDGE_MIN_CNT_W {
        UART_POSEDGE_MIN_CNT_W { w: self }
    }
}
