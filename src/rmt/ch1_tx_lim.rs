#[doc = "Reader of register CH1_TX_LIM"]
pub type R = crate::R<u32, super::CH1_TX_LIM>;
#[doc = "Writer for register CH1_TX_LIM"]
pub type W = crate::W<u32, super::CH1_TX_LIM>;
#[doc = "Register CH1_TX_LIM `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1_TX_LIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_LIM_CH1`"]
pub type TX_LIM_CH1_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX_LIM_CH1`"]
pub struct TX_LIM_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LIM_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - When channel1 sends more than reg_rmt_tx_lim_ch1 datas then channel1 produce the relative interrupt."]
    #[inline(always)]
    pub fn tx_lim_ch1(&self) -> TX_LIM_CH1_R {
        TX_LIM_CH1_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - When channel1 sends more than reg_rmt_tx_lim_ch1 datas then channel1 produce the relative interrupt."]
    #[inline(always)]
    pub fn tx_lim_ch1(&mut self) -> TX_LIM_CH1_W {
        TX_LIM_CH1_W { w: self }
    }
}
