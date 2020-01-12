#[doc = "Reader of register APP_DCACHE_DBUG0"]
pub type R = crate::R<u32, super::APP_DCACHE_DBUG0>;
#[doc = "Writer for register APP_DCACHE_DBUG0"]
pub type W = crate::W<u32, super::APP_DCACHE_DBUG0>;
#[doc = "Register APP_DCACHE_DBUG0 `reset()`'s with value 0"]
impl crate::ResetValue for super::APP_DCACHE_DBUG0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APP_RX_END`"]
pub type APP_RX_END_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_RX_END`"]
pub struct APP_RX_END_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_RX_END_W<'a> {
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
#[doc = "Reader of field `APP_SLAVE_WDATA_V`"]
pub type APP_SLAVE_WDATA_V_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_SLAVE_WDATA_V`"]
pub struct APP_SLAVE_WDATA_V_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_SLAVE_WDATA_V_W<'a> {
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
#[doc = "Reader of field `APP_SLAVE_WR`"]
pub type APP_SLAVE_WR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_SLAVE_WR`"]
pub struct APP_SLAVE_WR_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_SLAVE_WR_W<'a> {
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
#[doc = "Reader of field `APP_TX_END`"]
pub type APP_TX_END_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_TX_END`"]
pub struct APP_TX_END_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_TX_END_W<'a> {
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
#[doc = "Reader of field `APP_WR_BAK_TO_READ`"]
pub type APP_WR_BAK_TO_READ_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_WR_BAK_TO_READ`"]
pub struct APP_WR_BAK_TO_READ_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_WR_BAK_TO_READ_W<'a> {
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
#[doc = "Reader of field `APP_CACHE_STATE`"]
pub type APP_CACHE_STATE_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `APP_CACHE_STATE`"]
pub struct APP_CACHE_STATE_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_STATE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0fff << 7)) | (((value as u32) & 0x0fff) << 7);
        self.w
    }
}
#[doc = "Reader of field `APP_CACHE_IA`"]
pub type APP_CACHE_IA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `APP_CACHE_IA`"]
pub struct APP_CACHE_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 1)) | (((value as u32) & 0x3f) << 1);
        self.w
    }
}
#[doc = "Reader of field `APP_CACHE_MMU_IA`"]
pub type APP_CACHE_MMU_IA_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `APP_CACHE_MMU_IA`"]
pub struct APP_CACHE_MMU_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> APP_CACHE_MMU_IA_W<'a> {
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
    pub fn app_rx_end(&self) -> APP_RX_END_R {
        APP_RX_END_R::new(((self.bits >> 23) & 0x01) != 0)
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn app_slave_wdata_v(&self) -> APP_SLAVE_WDATA_V_R {
        APP_SLAVE_WDATA_V_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn app_slave_wr(&self) -> APP_SLAVE_WR_R {
        APP_SLAVE_WR_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn app_tx_end(&self) -> APP_TX_END_R {
        APP_TX_END_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn app_wr_bak_to_read(&self) -> APP_WR_BAK_TO_READ_R {
        APP_WR_BAK_TO_READ_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bits 7:18"]
    #[inline(always)]
    pub fn app_cache_state(&self) -> APP_CACHE_STATE_R {
        APP_CACHE_STATE_R::new(((self.bits >> 7) & 0x0fff) as u16)
    }
    #[doc = "Bits 1:6"]
    #[inline(always)]
    pub fn app_cache_ia(&self) -> APP_CACHE_IA_R {
        APP_CACHE_IA_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cache_mmu_ia(&self) -> APP_CACHE_MMU_IA_R {
        APP_CACHE_MMU_IA_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 23"]
    #[inline(always)]
    pub fn app_rx_end(&mut self) -> APP_RX_END_W {
        APP_RX_END_W { w: self }
    }
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn app_slave_wdata_v(&mut self) -> APP_SLAVE_WDATA_V_W {
        APP_SLAVE_WDATA_V_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn app_slave_wr(&mut self) -> APP_SLAVE_WR_W {
        APP_SLAVE_WR_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn app_tx_end(&mut self) -> APP_TX_END_W {
        APP_TX_END_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn app_wr_bak_to_read(&mut self) -> APP_WR_BAK_TO_READ_W {
        APP_WR_BAK_TO_READ_W { w: self }
    }
    #[doc = "Bits 7:18"]
    #[inline(always)]
    pub fn app_cache_state(&mut self) -> APP_CACHE_STATE_W {
        APP_CACHE_STATE_W { w: self }
    }
    #[doc = "Bits 1:6"]
    #[inline(always)]
    pub fn app_cache_ia(&mut self) -> APP_CACHE_IA_W {
        APP_CACHE_IA_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_cache_mmu_ia(&mut self) -> APP_CACHE_MMU_IA_W {
        APP_CACHE_MMU_IA_W { w: self }
    }
}
