#[doc = "Reader of register SAMPLE_RATE_CONF"]
pub type R = crate::R<u32, super::SAMPLE_RATE_CONF>;
#[doc = "Writer for register SAMPLE_RATE_CONF"]
pub type W = crate::W<u32, super::SAMPLE_RATE_CONF>;
#[doc = "Register SAMPLE_RATE_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::SAMPLE_RATE_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_BITS_MOD`"]
pub type RX_BITS_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_BITS_MOD`"]
pub struct RX_BITS_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BITS_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 18)) | (((value as u32) & 0x3f) << 18);
        self.w
    }
}
#[doc = "Reader of field `TX_BITS_MOD`"]
pub type TX_BITS_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_BITS_MOD`"]
pub struct TX_BITS_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BITS_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 12)) | (((value as u32) & 0x3f) << 12);
        self.w
    }
}
#[doc = "Reader of field `RX_BCK_DIV_NUM`"]
pub type RX_BCK_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_BCK_DIV_NUM`"]
pub struct RX_BCK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_BCK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `TX_BCK_DIV_NUM`"]
pub type TX_BCK_DIV_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_BCK_DIV_NUM`"]
pub struct TX_BCK_DIV_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_BCK_DIV_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn rx_bits_mod(&self) -> RX_BITS_MOD_R {
        RX_BITS_MOD_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn tx_bits_mod(&self) -> TX_BITS_MOD_R {
        TX_BITS_MOD_R::new(((self.bits >> 12) & 0x3f) as u8)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn rx_bck_div_num(&self) -> RX_BCK_DIV_NUM_R {
        RX_BCK_DIV_NUM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tx_bck_div_num(&self) -> TX_BCK_DIV_NUM_R {
        TX_BCK_DIV_NUM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 18:23"]
    #[inline(always)]
    pub fn rx_bits_mod(&mut self) -> RX_BITS_MOD_W {
        RX_BITS_MOD_W { w: self }
    }
    #[doc = "Bits 12:17"]
    #[inline(always)]
    pub fn tx_bits_mod(&mut self) -> TX_BITS_MOD_W {
        TX_BITS_MOD_W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn rx_bck_div_num(&mut self) -> RX_BCK_DIV_NUM_W {
        RX_BCK_DIV_NUM_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn tx_bck_div_num(&mut self) -> TX_BCK_DIV_NUM_W {
        TX_BCK_DIV_NUM_W { w: self }
    }
}
