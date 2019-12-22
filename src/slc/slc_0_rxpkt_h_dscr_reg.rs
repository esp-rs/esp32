#[doc = "Reader of register SLC_0_RXPKT_H_DSCR_REG"]
pub type R = crate::R<u32, super::SLC_0_RXPKT_H_DSCR_REG>;
#[doc = "Writer for register SLC_0_RXPKT_H_DSCR_REG"]
pub type W = crate::W<u32, super::SLC_0_RXPKT_H_DSCR_REG>;
#[doc = "Register SLC_0_RXPKT_H_DSCR_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SLC_0_RXPKT_H_DSCR_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_SLC0_RX_PKT_H_DSCR_ADDR`"]
pub type SLC_SLC0_RX_PKT_H_DSCR_ADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `SLC_SLC0_RX_PKT_H_DSCR_ADDR`"]
pub struct SLC_SLC0_RX_PKT_H_DSCR_ADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_RX_PKT_H_DSCR_ADDR_W<'a> {
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
    pub fn slc_slc0_rx_pkt_h_dscr_addr(&self) -> SLC_SLC0_RX_PKT_H_DSCR_ADDR_R {
        SLC_SLC0_RX_PKT_H_DSCR_ADDR_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn slc_slc0_rx_pkt_h_dscr_addr(&mut self) -> SLC_SLC0_RX_PKT_H_DSCR_ADDR_W {
        SLC_SLC0_RX_PKT_H_DSCR_ADDR_W { w: self }
    }
}