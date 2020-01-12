#[doc = "Reader of register AT_CMD_GAPTOUT"]
pub type R = crate::R<u32, super::AT_CMD_GAPTOUT>;
#[doc = "Writer for register AT_CMD_GAPTOUT"]
pub type W = crate::W<u32, super::AT_CMD_GAPTOUT>;
#[doc = "Register AT_CMD_GAPTOUT `reset()`'s with value 0"]
impl crate::ResetValue for super::AT_CMD_GAPTOUT {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_GAP_TOUT`"]
pub type RX_GAP_TOUT_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `RX_GAP_TOUT`"]
pub struct RX_GAP_TOUT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_GAP_TOUT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x00ff_ffff) | ((value as u32) & 0x00ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:23 - This register is used to configure the duration time between the at_cmd chars. when the duration time is less than this register value it will not take the datas as continous at_cmd chars."]
    #[inline(always)]
    pub fn rx_gap_tout(&self) -> RX_GAP_TOUT_R {
        RX_GAP_TOUT_R::new((self.bits & 0x00ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:23 - This register is used to configure the duration time between the at_cmd chars. when the duration time is less than this register value it will not take the datas as continous at_cmd chars."]
    #[inline(always)]
    pub fn rx_gap_tout(&mut self) -> RX_GAP_TOUT_W {
        RX_GAP_TOUT_W { w: self }
    }
}
