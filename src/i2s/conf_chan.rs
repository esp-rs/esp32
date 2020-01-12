#[doc = "Reader of register CONF_CHAN"]
pub type R = crate::R<u32, super::CONF_CHAN>;
#[doc = "Writer for register CONF_CHAN"]
pub type W = crate::W<u32, super::CONF_CHAN>;
#[doc = "Register CONF_CHAN `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF_CHAN {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_CHAN_MOD`"]
pub type RX_CHAN_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_CHAN_MOD`"]
pub struct RX_CHAN_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_CHAN_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 3)) | (((value as u32) & 0x03) << 3);
        self.w
    }
}
#[doc = "Reader of field `TX_CHAN_MOD`"]
pub type TX_CHAN_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_CHAN_MOD`"]
pub struct TX_CHAN_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_CHAN_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rx_chan_mod(&self) -> RX_CHAN_MOD_R {
        RX_CHAN_MOD_R::new(((self.bits >> 3) & 0x03) as u8)
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tx_chan_mod(&self) -> TX_CHAN_MOD_R {
        TX_CHAN_MOD_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:4"]
    #[inline(always)]
    pub fn rx_chan_mod(&mut self) -> RX_CHAN_MOD_W {
        RX_CHAN_MOD_W { w: self }
    }
    #[doc = "Bits 0:2"]
    #[inline(always)]
    pub fn tx_chan_mod(&mut self) -> TX_CHAN_MOD_W {
        TX_CHAN_MOD_W { w: self }
    }
}
