#[doc = "Reader of register UART_AT_CMD_PRECNT_REG(i)"]
pub type R = crate::R<u32, super::UART_AT_CMD_PRECNT_REGI>;
#[doc = "Writer for register UART_AT_CMD_PRECNT_REG(i)"]
pub type W = crate::W<u32, super::UART_AT_CMD_PRECNT_REGI>;
#[doc = "Register UART_AT_CMD_PRECNT_REG(i) `reset()`'s with value 0"]
impl crate::ResetValue for super::UART_AT_CMD_PRECNT_REGI {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `UART_PRE_IDLE_NUM`"]
pub type UART_PRE_IDLE_NUM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `UART_PRE_IDLE_NUM`"]
pub struct UART_PRE_IDLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> UART_PRE_IDLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - This register is used to configure the idle duration time before the first at_cmd is received by receiver. when the the duration is less than this register value it will not take the next data received as at_cmd char."]
    #[inline(always)]
    pub fn uart_pre_idle_num(&self) -> UART_PRE_IDLE_NUM_R {
        UART_PRE_IDLE_NUM_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - This register is used to configure the idle duration time before the first at_cmd is received by receiver. when the the duration is less than this register value it will not take the next data received as at_cmd char."]
    #[inline(always)]
    pub fn uart_pre_idle_num(&mut self) -> UART_PRE_IDLE_NUM_W {
        UART_PRE_IDLE_NUM_W { w: self }
    }
}
