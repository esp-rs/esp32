#[doc = "Reader of register SLC_BRIDGE_CONF_REG"]
pub type R = crate::R<u32, super::SLC_BRIDGE_CONF_REG>;
#[doc = "Writer for register SLC_BRIDGE_CONF_REG"]
pub type W = crate::W<u32, super::SLC_BRIDGE_CONF_REG>;
#[doc = "Register SLC_BRIDGE_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::SLC_BRIDGE_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `SLC_TX_PUSH_IDLE_NUM`"]
pub type SLC_TX_PUSH_IDLE_NUM_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `SLC_TX_PUSH_IDLE_NUM`"]
pub struct SLC_TX_PUSH_IDLE_NUM_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_TX_PUSH_IDLE_NUM_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 16)) | (((value as u32) & 0xffff) << 16);
        self.w
    }
}
#[doc = "Reader of field `SLC_SLC1_TX_DUMMY_MODE`"]
pub type SLC_SLC1_TX_DUMMY_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC1_TX_DUMMY_MODE`"]
pub struct SLC_SLC1_TX_DUMMY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC1_TX_DUMMY_MODE_W<'a> {
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
#[doc = "Reader of field `SLC_HDA_MAP_128K`"]
pub type SLC_HDA_MAP_128K_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_HDA_MAP_128K`"]
pub struct SLC_HDA_MAP_128K_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_HDA_MAP_128K_W<'a> {
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
#[doc = "Reader of field `SLC_SLC0_TX_DUMMY_MODE`"]
pub type SLC_SLC0_TX_DUMMY_MODE_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC_SLC0_TX_DUMMY_MODE`"]
pub struct SLC_SLC0_TX_DUMMY_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_SLC0_TX_DUMMY_MODE_W<'a> {
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
#[doc = "Reader of field `SLC_FIFO_MAP_ENA`"]
pub type SLC_FIFO_MAP_ENA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLC_FIFO_MAP_ENA`"]
pub struct SLC_FIFO_MAP_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_FIFO_MAP_ENA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 8)) | (((value as u32) & 0x0f) << 8);
        self.w
    }
}
#[doc = "Reader of field `SLC_TXEOF_ENA`"]
pub type SLC_TXEOF_ENA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `SLC_TXEOF_ENA`"]
pub struct SLC_TXEOF_ENA_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC_TXEOF_ENA_W<'a> {
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
    pub fn slc_tx_push_idle_num(&self) -> SLC_TX_PUSH_IDLE_NUM_R {
        SLC_TX_PUSH_IDLE_NUM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc_slc1_tx_dummy_mode(&self) -> SLC_SLC1_TX_DUMMY_MODE_R {
        SLC_SLC1_TX_DUMMY_MODE_R::new(((self.bits >> 14) & 0x01) != 0)
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc_hda_map_128k(&self) -> SLC_HDA_MAP_128K_R {
        SLC_HDA_MAP_128K_R::new(((self.bits >> 13) & 0x01) != 0)
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_slc0_tx_dummy_mode(&self) -> SLC_SLC0_TX_DUMMY_MODE_R {
        SLC_SLC0_TX_DUMMY_MODE_R::new(((self.bits >> 12) & 0x01) != 0)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn slc_fifo_map_ena(&self) -> SLC_FIFO_MAP_ENA_R {
        SLC_FIFO_MAP_ENA_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn slc_txeof_ena(&self) -> SLC_TXEOF_ENA_R {
        SLC_TXEOF_ENA_R::new((self.bits & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bits 16:31"]
    #[inline(always)]
    pub fn slc_tx_push_idle_num(&mut self) -> SLC_TX_PUSH_IDLE_NUM_W {
        SLC_TX_PUSH_IDLE_NUM_W { w: self }
    }
    #[doc = "Bit 14"]
    #[inline(always)]
    pub fn slc_slc1_tx_dummy_mode(&mut self) -> SLC_SLC1_TX_DUMMY_MODE_W {
        SLC_SLC1_TX_DUMMY_MODE_W { w: self }
    }
    #[doc = "Bit 13"]
    #[inline(always)]
    pub fn slc_hda_map_128k(&mut self) -> SLC_HDA_MAP_128K_W {
        SLC_HDA_MAP_128K_W { w: self }
    }
    #[doc = "Bit 12"]
    #[inline(always)]
    pub fn slc_slc0_tx_dummy_mode(&mut self) -> SLC_SLC0_TX_DUMMY_MODE_W {
        SLC_SLC0_TX_DUMMY_MODE_W { w: self }
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn slc_fifo_map_ena(&mut self) -> SLC_FIFO_MAP_ENA_W {
        SLC_FIFO_MAP_ENA_W { w: self }
    }
    #[doc = "Bits 0:5"]
    #[inline(always)]
    pub fn slc_txeof_ena(&mut self) -> SLC_TXEOF_ENA_W {
        SLC_TXEOF_ENA_W { w: self }
    }
}
