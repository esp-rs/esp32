#[doc = "Reader of register CONF1"]
pub type R = crate::R<u32, super::CONF1>;
#[doc = "Writer for register CONF1"]
pub type W = crate::W<u32, super::CONF1>;
#[doc = "Register CONF1 `reset()`'s with value 0"]
impl crate::ResetValue for super::CONF1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `CLK_EN`"]
pub type CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CLK_EN`"]
pub struct CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CLK_EN_W<'a> {
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
#[doc = "Reader of field `SLC1_RX_STITCH_EN`"]
pub type SLC1_RX_STITCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC1_RX_STITCH_EN`"]
pub struct SLC1_RX_STITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RX_STITCH_EN_W<'a> {
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
#[doc = "Reader of field `SLC1_TX_STITCH_EN`"]
pub type SLC1_TX_STITCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC1_TX_STITCH_EN`"]
pub struct SLC1_TX_STITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TX_STITCH_EN_W<'a> {
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
#[doc = "Reader of field `HOST_INT_LEVEL_SEL`"]
pub type HOST_INT_LEVEL_SEL_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `HOST_INT_LEVEL_SEL`"]
pub struct HOST_INT_LEVEL_SEL_W<'a> {
    w: &'a mut W,
}
impl<'a> HOST_INT_LEVEL_SEL_W<'a> {
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
#[doc = "Reader of field `SLC1_RX_CHECK_SUM_EN`"]
pub type SLC1_RX_CHECK_SUM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC1_RX_CHECK_SUM_EN`"]
pub struct SLC1_RX_CHECK_SUM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_RX_CHECK_SUM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 18)) | (((value as u32) & 0x01) << 18);
        self.w
    }
}
#[doc = "Reader of field `SLC1_TX_CHECK_SUM_EN`"]
pub type SLC1_TX_CHECK_SUM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC1_TX_CHECK_SUM_EN`"]
pub struct SLC1_TX_CHECK_SUM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_TX_CHECK_SUM_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 17)) | (((value as u32) & 0x01) << 17);
        self.w
    }
}
#[doc = "Reader of field `SLC1_CHECK_OWNER`"]
pub type SLC1_CHECK_OWNER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC1_CHECK_OWNER`"]
pub struct SLC1_CHECK_OWNER_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC1_CHECK_OWNER_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 16)) | (((value as u32) & 0x01) << 16);
        self.w
    }
}
#[doc = "Reader of field `SLC0_RX_STITCH_EN`"]
pub type SLC0_RX_STITCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_RX_STITCH_EN`"]
pub struct SLC0_RX_STITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_STITCH_EN_W<'a> {
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
#[doc = "Reader of field `SLC0_TX_STITCH_EN`"]
pub type SLC0_TX_STITCH_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_TX_STITCH_EN`"]
pub struct SLC0_TX_STITCH_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_STITCH_EN_W<'a> {
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
#[doc = "Reader of field `SLC0_LEN_AUTO_CLR`"]
pub type SLC0_LEN_AUTO_CLR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_LEN_AUTO_CLR`"]
pub struct SLC0_LEN_AUTO_CLR_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_LEN_AUTO_CLR_W<'a> {
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
#[doc = "Reader of field `CMD_HOLD_EN`"]
pub type CMD_HOLD_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `CMD_HOLD_EN`"]
pub struct CMD_HOLD_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> CMD_HOLD_EN_W<'a> {
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
#[doc = "Reader of field `SLC0_RX_CHECK_SUM_EN`"]
pub type SLC0_RX_CHECK_SUM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_RX_CHECK_SUM_EN`"]
pub struct SLC0_RX_CHECK_SUM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_RX_CHECK_SUM_EN_W<'a> {
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
#[doc = "Reader of field `SLC0_TX_CHECK_SUM_EN`"]
pub type SLC0_TX_CHECK_SUM_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_TX_CHECK_SUM_EN`"]
pub struct SLC0_TX_CHECK_SUM_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_TX_CHECK_SUM_EN_W<'a> {
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
#[doc = "Reader of field `SLC0_CHECK_OWNER`"]
pub type SLC0_CHECK_OWNER_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `SLC0_CHECK_OWNER`"]
pub struct SLC0_CHECK_OWNER_W<'a> {
    w: &'a mut W,
}
impl<'a> SLC0_CHECK_OWNER_W<'a> {
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
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_en(&self) -> CLK_EN_R {
        CLK_EN_R::new(((self.bits >> 22) & 0x01) != 0)
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc1_rx_stitch_en(&self) -> SLC1_RX_STITCH_EN_R {
        SLC1_RX_STITCH_EN_R::new(((self.bits >> 21) & 0x01) != 0)
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_tx_stitch_en(&self) -> SLC1_TX_STITCH_EN_R {
        SLC1_TX_STITCH_EN_R::new(((self.bits >> 20) & 0x01) != 0)
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn host_int_level_sel(&self) -> HOST_INT_LEVEL_SEL_R {
        HOST_INT_LEVEL_SEL_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_rx_check_sum_en(&self) -> SLC1_RX_CHECK_SUM_EN_R {
        SLC1_RX_CHECK_SUM_EN_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_tx_check_sum_en(&self) -> SLC1_TX_CHECK_SUM_EN_R {
        SLC1_TX_CHECK_SUM_EN_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_check_owner(&self) -> SLC1_CHECK_OWNER_R {
        SLC1_CHECK_OWNER_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc0_rx_stitch_en(&self) -> SLC0_RX_STITCH_EN_R {
        SLC0_RX_STITCH_EN_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc0_tx_stitch_en(&self) -> SLC0_TX_STITCH_EN_R {
        SLC0_TX_STITCH_EN_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_len_auto_clr(&self) -> SLC0_LEN_AUTO_CLR_R {
        SLC0_LEN_AUTO_CLR_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmd_hold_en(&self) -> CMD_HOLD_EN_R {
        CMD_HOLD_EN_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc0_rx_check_sum_en(&self) -> SLC0_RX_CHECK_SUM_EN_R {
        SLC0_RX_CHECK_SUM_EN_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_tx_check_sum_en(&self) -> SLC0_TX_CHECK_SUM_EN_R {
        SLC0_TX_CHECK_SUM_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_check_owner(&self) -> SLC0_CHECK_OWNER_R {
        SLC0_CHECK_OWNER_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 22"]
    #[inline(always)]
    pub fn clk_en(&mut self) -> CLK_EN_W {
        CLK_EN_W { w: self }
    }
    #[doc = "Bit 21"]
    #[inline(always)]
    pub fn slc1_rx_stitch_en(&mut self) -> SLC1_RX_STITCH_EN_W {
        SLC1_RX_STITCH_EN_W { w: self }
    }
    #[doc = "Bit 20"]
    #[inline(always)]
    pub fn slc1_tx_stitch_en(&mut self) -> SLC1_TX_STITCH_EN_W {
        SLC1_TX_STITCH_EN_W { w: self }
    }
    #[doc = "Bit 19"]
    #[inline(always)]
    pub fn host_int_level_sel(&mut self) -> HOST_INT_LEVEL_SEL_W {
        HOST_INT_LEVEL_SEL_W { w: self }
    }
    #[doc = "Bit 18"]
    #[inline(always)]
    pub fn slc1_rx_check_sum_en(&mut self) -> SLC1_RX_CHECK_SUM_EN_W {
        SLC1_RX_CHECK_SUM_EN_W { w: self }
    }
    #[doc = "Bit 17"]
    #[inline(always)]
    pub fn slc1_tx_check_sum_en(&mut self) -> SLC1_TX_CHECK_SUM_EN_W {
        SLC1_TX_CHECK_SUM_EN_W { w: self }
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn slc1_check_owner(&mut self) -> SLC1_CHECK_OWNER_W {
        SLC1_CHECK_OWNER_W { w: self }
    }
    #[doc = "Bit 6"]
    #[inline(always)]
    pub fn slc0_rx_stitch_en(&mut self) -> SLC0_RX_STITCH_EN_W {
        SLC0_RX_STITCH_EN_W { w: self }
    }
    #[doc = "Bit 5"]
    #[inline(always)]
    pub fn slc0_tx_stitch_en(&mut self) -> SLC0_TX_STITCH_EN_W {
        SLC0_TX_STITCH_EN_W { w: self }
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn slc0_len_auto_clr(&mut self) -> SLC0_LEN_AUTO_CLR_W {
        SLC0_LEN_AUTO_CLR_W { w: self }
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn cmd_hold_en(&mut self) -> CMD_HOLD_EN_W {
        CMD_HOLD_EN_W { w: self }
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn slc0_rx_check_sum_en(&mut self) -> SLC0_RX_CHECK_SUM_EN_W {
        SLC0_RX_CHECK_SUM_EN_W { w: self }
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn slc0_tx_check_sum_en(&mut self) -> SLC0_TX_CHECK_SUM_EN_W {
        SLC0_TX_CHECK_SUM_EN_W { w: self }
    }
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn slc0_check_owner(&mut self) -> SLC0_CHECK_OWNER_W {
        SLC0_CHECK_OWNER_W { w: self }
    }
}
