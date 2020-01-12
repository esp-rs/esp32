#[doc = "Reader of register ESCAPE_CONF"]
pub type R = crate::R<u32, super::ESCAPE_CONF>;
#[doc = "Writer for register ESCAPE_CONF"]
pub type W = crate::W<u32, super::ESCAPE_CONF>;
#[doc = "Register ESCAPE_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::ESCAPE_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RX_13_ESC_EN`"]
pub type RX_13_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_13_ESC_EN`"]
pub struct RX_13_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_13_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 7)) | (((value as u32) & 0x01) << 7);
        self.w
    }
}
#[doc = "Reader of field `RX_11_ESC_EN`"]
pub type RX_11_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_11_ESC_EN`"]
pub struct RX_11_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_11_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 6)) | (((value as u32) & 0x01) << 6);
        self.w
    }
}
#[doc = "Reader of field `RX_DB_ESC_EN`"]
pub type RX_DB_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_DB_ESC_EN`"]
pub struct RX_DB_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_DB_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 5)) | (((value as u32) & 0x01) << 5);
        self.w
    }
}
#[doc = "Reader of field `RX_C0_ESC_EN`"]
pub type RX_C0_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RX_C0_ESC_EN`"]
pub struct RX_C0_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_C0_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `TX_13_ESC_EN`"]
pub type TX_13_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_13_ESC_EN`"]
pub struct TX_13_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_13_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `TX_11_ESC_EN`"]
pub type TX_11_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_11_ESC_EN`"]
pub struct TX_11_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_11_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 2)) | (((value as u32) & 0x01) << 2);
        self.w
    }
}
#[doc = "Reader of field `TX_DB_ESC_EN`"]
pub type TX_DB_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_DB_ESC_EN`"]
pub struct TX_DB_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_DB_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 1)) | (((value as u32) & 0x01) << 1);
        self.w
    }
}
#[doc = "Reader of field `TX_C0_ESC_EN`"]
pub type TX_C0_ESC_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `TX_C0_ESC_EN`"]
pub struct TX_C0_ESC_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_C0_ESC_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 7 - Set this bit to enable flow control char 0x13 replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_13_esc_en(&self) -> RX_13_ESC_EN_R {
        RX_13_ESC_EN_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set this bit to enable flow control char 0x11 replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_11_esc_en(&self) -> RX_11_ESC_EN_R {
        RX_11_ESC_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - Set this bit to enable 0xdb char replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_db_esc_en(&self) -> RX_DB_ESC_EN_R {
        RX_DB_ESC_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this bit to enable 0xc0 char replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_c0_esc_en(&self) -> RX_C0_ESC_EN_R {
        RX_C0_ESC_EN_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this bit to enable flow control char 0x13 decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_13_esc_en(&self) -> TX_13_ESC_EN_R {
        TX_13_ESC_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to enable flow control char 0x11 decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_11_esc_en(&self) -> TX_11_ESC_EN_R {
        TX_11_ESC_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enable 0xdb char decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_db_esc_en(&self) -> TX_DB_ESC_EN_R {
        TX_DB_ESC_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set this bit to enable 0xc0 char decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_c0_esc_en(&self) -> TX_C0_ESC_EN_R {
        TX_C0_ESC_EN_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 7 - Set this bit to enable flow control char 0x13 replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_13_esc_en(&mut self) -> RX_13_ESC_EN_W {
        RX_13_ESC_EN_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to enable flow control char 0x11 replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_11_esc_en(&mut self) -> RX_11_ESC_EN_W {
        RX_11_ESC_EN_W { w: self }
    }
    #[doc = "Bit 5 - Set this bit to enable 0xdb char replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_db_esc_en(&mut self) -> RX_DB_ESC_EN_W {
        RX_DB_ESC_EN_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to enable 0xc0 char replace when DMA sends data."]
    #[inline(always)]
    pub fn rx_c0_esc_en(&mut self) -> RX_C0_ESC_EN_W {
        RX_C0_ESC_EN_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to enable flow control char 0x13 decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_13_esc_en(&mut self) -> TX_13_ESC_EN_W {
        TX_13_ESC_EN_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to enable flow control char 0x11 decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_11_esc_en(&mut self) -> TX_11_ESC_EN_W {
        TX_11_ESC_EN_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to enable 0xdb char decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_db_esc_en(&mut self) -> TX_DB_ESC_EN_W {
        TX_DB_ESC_EN_W { w: self }
    }
    #[doc = "Bit 0 - Set this bit to enable 0xc0 char decode when DMA receives data."]
    #[inline(always)]
    pub fn tx_c0_esc_en(&mut self) -> TX_C0_ESC_EN_W {
        TX_C0_ESC_EN_W { w: self }
    }
}
