#[doc = "Reader of register RXEOF_NUM"]
pub type R = crate::R<u32, super::RXEOF_NUM>;
#[doc = "Writer for register RXEOF_NUM"]
pub type W = crate::W<u32, super::RXEOF_NUM>;
#[doc = "Register RXEOF_NUM `reset()`'s with value 0"]
impl crate::ResetValue for super::RXEOF_NUM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `I2S_RX_EOF_NUM`"]
pub type I2S_RX_EOF_NUM_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `I2S_RX_EOF_NUM`"]
pub struct I2S_RX_EOF_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> I2S_RX_EOF_NUM_W<'a> {
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
    pub fn i2s_rx_eof_num(&self) -> I2S_RX_EOF_NUM_R {
        I2S_RX_EOF_NUM_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn i2s_rx_eof_num(&mut self) -> I2S_RX_EOF_NUM_W {
        I2S_RX_EOF_NUM_W { w: self }
    }
}
