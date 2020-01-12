#[doc = "Reader of register CH2_TX_LIM"]
pub type R = crate::R<u32, super::CH2_TX_LIM>;
#[doc = "Writer for register CH2_TX_LIM"]
pub type W = crate::W<u32, super::CH2_TX_LIM>;
#[doc = "Register CH2_TX_LIM `reset()`'s with value 0"]
impl crate::ResetValue for super::CH2_TX_LIM {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_LIM_CH2`"]
pub type TX_LIM_CH2_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX_LIM_CH2`"]
pub struct TX_LIM_CH2_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_LIM_CH2_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x01ff) | ((value as u32) & 0x01ff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:8 - When channel2 sends more than reg_rmt_tx_lim_ch2 datas then channel2 produce the relative interrupt."]
    #[inline(always)]
    pub fn tx_lim_ch2(&self) -> TX_LIM_CH2_R {
        TX_LIM_CH2_R::new((self.bits & 0x01ff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:8 - When channel2 sends more than reg_rmt_tx_lim_ch2 datas then channel2 produce the relative interrupt."]
    #[inline(always)]
    pub fn tx_lim_ch2(&mut self) -> TX_LIM_CH2_W {
        TX_LIM_CH2_W { w: self }
    }
}
