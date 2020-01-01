#[doc = "Reader of register CH6_TX_LIM"]
pub type R = crate::R<u32, super::CH6_TX_LIM>;
#[doc = "Writer for register CH6_TX_LIM"]
pub type W = crate::W<u32, super::CH6_TX_LIM>;
#[doc = "Register CH6_TX_LIM `reset()`'s with value 0"]
impl crate::ResetValue for super::CH6_TX_LIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMT_TX_LIM_CH6`"]
pub type RMT_TX_LIM_CH6_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RMT_TX_LIM_CH6`"]
pub struct RMT_TX_LIM_CH6_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_TX_LIM_CH6_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - When channel6 sends more than reg_rmt_tx_lim_ch6 datas then channel6 produce the relative interrupt."]
    #[inline(always)]
    pub fn rmt_tx_lim_ch6(&self) -> RMT_TX_LIM_CH6_R {
        RMT_TX_LIM_CH6_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - When channel6 sends more than reg_rmt_tx_lim_ch6 datas then channel6 produce the relative interrupt."]
    #[inline(always)]
    pub fn rmt_tx_lim_ch6(&mut self) -> RMT_TX_LIM_CH6_W {
        RMT_TX_LIM_CH6_W { w: self }
    }
}
