#[doc = "Reader of register RMT_CH4_TX_LIM_REG"]
pub type R = crate::R<u32, super::RMT_CH4_TX_LIM_REG>;
#[doc = "Writer for register RMT_CH4_TX_LIM_REG"]
pub type W = crate::W<u32, super::RMT_CH4_TX_LIM_REG>;
#[doc = "Register RMT_CH4_TX_LIM_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RMT_CH4_TX_LIM_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMT_TX_LIM_CH4`"]
pub type RMT_TX_LIM_CH4_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RMT_TX_LIM_CH4`"]
pub struct RMT_TX_LIM_CH4_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_TX_LIM_CH4_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - When channel4 sends more than reg_rmt_tx_lim_ch4 datas then channel4 produce the relative interrupt."]
    #[inline(always)]
    pub fn rmt_tx_lim_ch4(&self) -> RMT_TX_LIM_CH4_R {
        RMT_TX_LIM_CH4_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - When channel4 sends more than reg_rmt_tx_lim_ch4 datas then channel4 produce the relative interrupt."]
    #[inline(always)]
    pub fn rmt_tx_lim_ch4(&mut self) -> RMT_TX_LIM_CH4_W {
        RMT_TX_LIM_CH4_W { w: self }
    }
}
