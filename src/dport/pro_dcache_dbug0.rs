#[doc = "Reader of register PRO_DCACHE_DBUG0"]
pub type R = crate::R<u32, super::PRO_DCACHE_DBUG0>;
#[doc = "Writer for register PRO_DCACHE_DBUG0"]
pub type W = crate::W<u32, super::PRO_DCACHE_DBUG0>;
#[doc = "Register PRO_DCACHE_DBUG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_DCACHE_DBUG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRO_RX_END`"]
pub type PRO_RX_END_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRO_RX_END`"]
pub struct PRO_RX_END_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_RX_END_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 23)) | (((value as u32) & 0x01) << 23);
        self.w
    }
}
#[doc = "Reader of field `PRO_SLAVE_WDATA_V`"]
pub type PRO_SLAVE_WDATA_V_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRO_SLAVE_WDATA_V`"]
pub struct PRO_SLAVE_WDATA_V_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_SLAVE_WDATA_V_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 22)) | (((value as u32) & 0x01) << 22);
        self.w
    }
}
#[doc = "Reader of field `PRO_SLAVE_WR`"]
pub type PRO_SLAVE_WR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRO_SLAVE_WR`"]
pub struct PRO_SLAVE_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_SLAVE_WR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 21)) | (((value as u32) & 0x01) << 21);
        self.w
    }
}
#[doc = "Reader of field `PRO_TX_END`"]
pub type PRO_TX_END_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRO_TX_END`"]
pub struct PRO_TX_END_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_TX_END_W<'a> {
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
#[doc = "Reader of field `PRO_WR_BAK_TO_READ`"]
pub type PRO_WR_BAK_TO_READ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRO_WR_BAK_TO_READ`"]
pub struct PRO_WR_BAK_TO_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_WR_BAK_TO_READ_W<'a> {
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
#[doc = "Reader of field `PRO_CACHE_STATE`"]
pub type PRO_CACHE_STATE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `PRO_CACHE_STATE`"]
pub struct PRO_CACHE_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 7)) | (((value as u32) & 0x0fff) << 7);
        self.w
    }
}
#[doc = "Reader of field `PRO_CACHE_IA`"]
pub type PRO_CACHE_IA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRO_CACHE_IA`"]
pub struct PRO_CACHE_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 1)) | (((value as u32) & 0x3f) << 1);
        self.w
    }
}
#[doc = "Reader of field `PRO_CACHE_MMU_IA`"]
pub type PRO_CACHE_MMU_IA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `PRO_CACHE_MMU_IA`"]
pub struct PRO_CACHE_MMU_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_MMU_IA_W<'a> {
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
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pro_rx_end(&self) -> PRO_RX_END_R {
        PRO_RX_END_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pro_slave_wdata_v(&self) -> PRO_SLAVE_WDATA_V_R {
        PRO_SLAVE_WDATA_V_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pro_slave_wr(&self) -> PRO_SLAVE_WR_R {
        PRO_SLAVE_WR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pro_tx_end(&self) -> PRO_TX_END_R {
        PRO_TX_END_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pro_wr_bak_to_read(&self) -> PRO_WR_BAK_TO_READ_R {
        PRO_WR_BAK_TO_READ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 7:18"]
    #[inline(always)]
    pub fn pro_cache_state(&self) -> PRO_CACHE_STATE_R {
        PRO_CACHE_STATE_R::new(((self.bits >> 7) & 0x0fff) as u16)
    }
    #[doc = "Bits 1:6"]
    #[inline(always)]
    pub fn pro_cache_ia(&self) -> PRO_CACHE_IA_R {
        PRO_CACHE_IA_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_cache_mmu_ia(&self) -> PRO_CACHE_MMU_IA_R {
        PRO_CACHE_MMU_IA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn pro_rx_end(&mut self) -> PRO_RX_END_W {
        PRO_RX_END_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn pro_slave_wdata_v(&mut self) -> PRO_SLAVE_WDATA_V_W {
        PRO_SLAVE_WDATA_V_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn pro_slave_wr(&mut self) -> PRO_SLAVE_WR_W {
        PRO_SLAVE_WR_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn pro_tx_end(&mut self) -> PRO_TX_END_W {
        PRO_TX_END_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn pro_wr_bak_to_read(&mut self) -> PRO_WR_BAK_TO_READ_W {
        PRO_WR_BAK_TO_READ_W { w: self }
    }
    #[doc = "Bits 7:18"]
    #[inline(always)]
    pub fn pro_cache_state(&mut self) -> PRO_CACHE_STATE_W {
        PRO_CACHE_STATE_W { w: self }
    }
    #[doc = "Bits 1:6"]
    #[inline(always)]
    pub fn pro_cache_ia(&mut self) -> PRO_CACHE_IA_W {
        PRO_CACHE_IA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn pro_cache_mmu_ia(&mut self) -> PRO_CACHE_MMU_IA_W {
        PRO_CACHE_MMU_IA_W { w: self }
    }
}
