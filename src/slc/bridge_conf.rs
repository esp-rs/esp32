#[doc = "Reader of register BRIDGE_CONF"]
pub type R = crate::R<u32, super::BRIDGE_CONF>;
#[doc = "Writer for register BRIDGE_CONF"]
pub type W = crate::W<u32, super::BRIDGE_CONF>;
#[doc = "Register BRIDGE_CONF `reset()`'s with value 0"]
impl crate::ResetValue for super::BRIDGE_CONF {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_PUSH_IDLE_NUM`"]
pub type TX_PUSH_IDLE_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `TX_PUSH_IDLE_NUM`"]
pub struct TX_PUSH_IDLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_PUSH_IDLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SLC1_TX_DUMMY_MODE`"]
pub type SLC1_TX_DUMMY_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC1_TX_DUMMY_MODE`"]
pub struct SLC1_TX_DUMMY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TX_DUMMY_MODE_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 14)) | (((value as u32) & 0x01) << 14);
        self.w
    }
}
#[doc = "Reader of field `HDA_MAP_128K`"]
pub type HDA_MAP_128K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HDA_MAP_128K`"]
pub struct HDA_MAP_128K_W<'a> {
    w: &'a mut W,
}
impl<'a> HDA_MAP_128K_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 13)) | (((value as u32) & 0x01) << 13);
        self.w
    }
}
#[doc = "Reader of field `SLC0_TX_DUMMY_MODE`"]
pub type SLC0_TX_DUMMY_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_TX_DUMMY_MODE`"]
pub struct SLC0_TX_DUMMY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_DUMMY_MODE_W<'a> {
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
#[doc = "Reader of field `FIFO_MAP_ENA`"]
pub type FIFO_MAP_ENA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FIFO_MAP_ENA`"]
pub struct FIFO_MAP_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> FIFO_MAP_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `TXEOF_ENA`"]
pub type TXEOF_ENA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TXEOF_ENA`"]
pub struct TXEOF_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> TXEOF_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x3f) | ((value as u32) & 0x3f);
        self.w
    }
}
impl R {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tx_push_idle_num(&self) -> TX_PUSH_IDLE_NUM_R {
        TX_PUSH_IDLE_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc1_tx_dummy_mode(&self) -> SLC1_TX_DUMMY_MODE_R {
        SLC1_TX_DUMMY_MODE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hda_map_128k(&self) -> HDA_MAP_128K_R {
        HDA_MAP_128K_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc0_tx_dummy_mode(&self) -> SLC0_TX_DUMMY_MODE_R {
        SLC0_TX_DUMMY_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn fifo_map_ena(&self) -> FIFO_MAP_ENA_R {
        FIFO_MAP_ENA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn txeof_ena(&self) -> TXEOF_ENA_R {
        TXEOF_ENA_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn tx_push_idle_num(&mut self) -> TX_PUSH_IDLE_NUM_W {
        TX_PUSH_IDLE_NUM_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc1_tx_dummy_mode(&mut self) -> SLC1_TX_DUMMY_MODE_W {
        SLC1_TX_DUMMY_MODE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn hda_map_128k(&mut self) -> HDA_MAP_128K_W {
        HDA_MAP_128K_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc0_tx_dummy_mode(&mut self) -> SLC0_TX_DUMMY_MODE_W {
        SLC0_TX_DUMMY_MODE_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn fifo_map_ena(&mut self) -> FIFO_MAP_ENA_W {
        FIFO_MAP_ENA_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn txeof_ena(&mut self) -> TXEOF_ENA_W {
        TXEOF_ENA_W { w: self }
    }
}
