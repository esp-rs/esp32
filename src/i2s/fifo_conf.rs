#[doc = "Reader of register FIFO_CONF"]
pub type R = crate::R<u32, super::FIFO_CONF>;
#[doc = "Writer for register FIFO_CONF"]
pub type W = crate::W<u32, super::FIFO_CONF>;
#[doc = "Register FIFO_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::FIFO_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_FIFO_MOD_FORCE_EN`"]
pub type RX_FIFO_MOD_FORCE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_FIFO_MOD_FORCE_EN`"]
pub struct RX_FIFO_MOD_FORCE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_MOD_FORCE_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 20)) | (((value as u32) & 0x01) << 20);
        self.w
    }
}
#[doc = "Reader of field `TX_FIFO_MOD_FORCE_EN`"]
pub type TX_FIFO_MOD_FORCE_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_FIFO_MOD_FORCE_EN`"]
pub struct TX_FIFO_MOD_FORCE_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_MOD_FORCE_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 19)) | (((value as u32) & 0x01) << 19);
        self.w
    }
}
#[doc = "Reader of field `RX_FIFO_MOD`"]
pub type RX_FIFO_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_FIFO_MOD`"]
pub struct RX_FIFO_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_FIFO_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 16)) | (((value as u32) & 0x07) << 16);
        self.w
    }
}
#[doc = "Reader of field `TX_FIFO_MOD`"]
pub type TX_FIFO_MOD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_FIFO_MOD`"]
pub struct TX_FIFO_MOD_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_FIFO_MOD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 13)) | (((value as u32) & 0x07) << 13);
        self.w
    }
}
#[doc = "Reader of field `DSCR_EN`"]
pub type DSCR_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DSCR_EN`"]
pub struct DSCR_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> DSCR_EN_W<'a> {
    #[doc = r"Sets the field bit"]
    #[inline(always)]
    pub fn set_bit(self) -> &'a mut W {
        self.bit(true)
    }
    #[doc = r"Clears the field bit"]
    #[inline(always)]
    pub fn clear_bit(self) -> &'a mut W {
        self.bit(false)
    }
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub fn bit(self, value: bool) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x01 << 12)) | (((value as u32) & 0x01) << 12);
        self.w
    }
}
#[doc = "Reader of field `TX_DATA_NUM`"]
pub type TX_DATA_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_DATA_NUM`"]
pub struct TX_DATA_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DATA_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 6)) | (((value as u32) & 0x3f) << 6);
        self.w
    }
}
#[doc = "Reader of field `RX_DATA_NUM`"]
pub type RX_DATA_NUM_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_DATA_NUM`"]
pub struct RX_DATA_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DATA_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_fifo_mod_force_en(&self) -> RX_FIFO_MOD_FORCE_EN_R {
        RX_FIFO_MOD_FORCE_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tx_fifo_mod_force_en(&self) -> TX_FIFO_MOD_FORCE_EN_R {
        TX_FIFO_MOD_FORCE_EN_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rx_fifo_mod(&self) -> RX_FIFO_MOD_R {
        RX_FIFO_MOD_R::new(((self.bits >> 16) & 0x07) as u8)
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn tx_fifo_mod(&self) -> TX_FIFO_MOD_R {
        TX_FIFO_MOD_R::new(((self.bits >> 13) & 0x07) as u8)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dscr_en(&self) -> DSCR_EN_R {
        DSCR_EN_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn tx_data_num(&self) -> TX_DATA_NUM_R {
        TX_DATA_NUM_R::new(((self.bits >> 6) & 0x3f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rx_data_num(&self) -> RX_DATA_NUM_R {
        RX_DATA_NUM_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn rx_fifo_mod_force_en(&mut self) -> RX_FIFO_MOD_FORCE_EN_W {
        RX_FIFO_MOD_FORCE_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn tx_fifo_mod_force_en(&mut self) -> TX_FIFO_MOD_FORCE_EN_W {
        TX_FIFO_MOD_FORCE_EN_W { w: self }
    }
    #[doc = "Bits 16:18"]
    #[inline(always)]
    pub fn rx_fifo_mod(&mut self) -> RX_FIFO_MOD_W {
        RX_FIFO_MOD_W { w: self }
    }
    #[doc = "Bits 13:15"]
    #[inline(always)]
    pub fn tx_fifo_mod(&mut self) -> TX_FIFO_MOD_W {
        TX_FIFO_MOD_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn dscr_en(&mut self) -> DSCR_EN_W {
        DSCR_EN_W { w: self }
    }
    #[doc = "Bits 6:11"]
    #[inline(always)]
    pub fn tx_data_num(&mut self) -> TX_DATA_NUM_W {
        TX_DATA_NUM_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn rx_data_num(&mut self) -> RX_DATA_NUM_W {
        RX_DATA_NUM_W { w: self }
    }
}
