#[doc = "Reader of register RMT_CH0CONF1_REG"]
pub type R = crate::R<u32, super::RMT_CH0CONF1_REG>;
#[doc = "Writer for register RMT_CH0CONF1_REG"]
pub type W = crate::W<u32, super::RMT_CH0CONF1_REG>;
#[doc = "Register RMT_CH0CONF1_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RMT_CH0CONF1_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMT_IDLE_OUT_EN_CH0`"]
pub type RMT_IDLE_OUT_EN_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_IDLE_OUT_EN_CH0`"]
pub struct RMT_IDLE_OUT_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_IDLE_OUT_EN_CH0_W<'a> {
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
#[doc = "Reader of field `RMT_IDLE_OUT_LV_CH0`"]
pub type RMT_IDLE_OUT_LV_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_IDLE_OUT_LV_CH0`"]
pub struct RMT_IDLE_OUT_LV_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_IDLE_OUT_LV_CH0_W<'a> {
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
#[doc = "Reader of field `RMT_REF_ALWAYS_ON_CH0`"]
pub type RMT_REF_ALWAYS_ON_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_REF_ALWAYS_ON_CH0`"]
pub struct RMT_REF_ALWAYS_ON_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_REF_ALWAYS_ON_CH0_W<'a> {
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
#[doc = "Reader of field `RMT_REF_CNT_RST_CH0`"]
pub type RMT_REF_CNT_RST_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_REF_CNT_RST_CH0`"]
pub struct RMT_REF_CNT_RST_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_REF_CNT_RST_CH0_W<'a> {
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
#[doc = "Reader of field `RMT_RX_FILTER_THRES_CH0`"]
pub type RMT_RX_FILTER_THRES_CH0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMT_RX_FILTER_THRES_CH0`"]
pub struct RMT_RX_FILTER_THRES_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_RX_FILTER_THRES_CH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xff << 8)) | (((value as u32) & 0xff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RMT_RX_FILTER_EN_CH0`"]
pub type RMT_RX_FILTER_EN_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_RX_FILTER_EN_CH0`"]
pub struct RMT_RX_FILTER_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_RX_FILTER_EN_CH0_W<'a> {
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
#[doc = "Reader of field `RMT_TX_CONTI_MODE_CH0`"]
pub type RMT_TX_CONTI_MODE_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_TX_CONTI_MODE_CH0`"]
pub struct RMT_TX_CONTI_MODE_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_TX_CONTI_MODE_CH0_W<'a> {
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
#[doc = "Reader of field `RMT_MEM_OWNER_CH0`"]
pub type RMT_MEM_OWNER_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_MEM_OWNER_CH0`"]
pub struct RMT_MEM_OWNER_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_OWNER_CH0_W<'a> {
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
#[doc = "Reader of field `RMT_APB_MEM_RST_CH0`"]
pub type RMT_APB_MEM_RST_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_APB_MEM_RST_CH0`"]
pub struct RMT_APB_MEM_RST_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_APB_MEM_RST_CH0_W<'a> {
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
#[doc = "Reader of field `RMT_MEM_RD_RST_CH0`"]
pub type RMT_MEM_RD_RST_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_MEM_RD_RST_CH0`"]
pub struct RMT_MEM_RD_RST_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_RD_RST_CH0_W<'a> {
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
#[doc = "Reader of field `RMT_MEM_WR_RST_CH0`"]
pub type RMT_MEM_WR_RST_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_MEM_WR_RST_CH0`"]
pub struct RMT_MEM_WR_RST_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_WR_RST_CH0_W<'a> {
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
#[doc = "Reader of field `RMT_RX_EN_CH0`"]
pub type RMT_RX_EN_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_RX_EN_CH0`"]
pub struct RMT_RX_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_RX_EN_CH0_W<'a> {
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
#[doc = "Reader of field `RMT_TX_START_CH0`"]
pub type RMT_TX_START_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_TX_START_CH0`"]
pub struct RMT_TX_START_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_TX_START_CH0_W<'a> {
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
    #[doc = "Bit 19 - This is the output enable control bit for channel0 in IDLE state."]
    #[inline(always)]
    pub fn rmt_idle_out_en_ch0(&self) -> RMT_IDLE_OUT_EN_CH0_R {
        RMT_IDLE_OUT_EN_CH0_R::new(((self.bits >> 19) & 0x01) != 0)
    }
    #[doc = "Bit 18 - This bit configures the output signal's level for channel0 in IDLE state."]
    #[inline(always)]
    pub fn rmt_idle_out_lv_ch0(&self) -> RMT_IDLE_OUT_LV_CH0_R {
        RMT_IDLE_OUT_LV_CH0_R::new(((self.bits >> 18) & 0x01) != 0)
    }
    #[doc = "Bit 17 - This bit is used to select base clock. 1'b1:clk_apb 1'b0:clk_ref"]
    #[inline(always)]
    pub fn rmt_ref_always_on_ch0(&self) -> RMT_REF_ALWAYS_ON_CH0_R {
        RMT_REF_ALWAYS_ON_CH0_R::new(((self.bits >> 17) & 0x01) != 0)
    }
    #[doc = "Bit 16 - This bit is used to reset divider in channel0."]
    #[inline(always)]
    pub fn rmt_ref_cnt_rst_ch0(&self) -> RMT_REF_CNT_RST_CH0_R {
        RMT_REF_CNT_RST_CH0_R::new(((self.bits >> 16) & 0x01) != 0)
    }
    #[doc = "Bits 8:15 - in receive mode channel0 ignore input pulse when the pulse width is smaller then this value."]
    #[inline(always)]
    pub fn rmt_rx_filter_thres_ch0(&self) -> RMT_RX_FILTER_THRES_CH0_R {
        RMT_RX_FILTER_THRES_CH0_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 7 - This is the receive filter enable bit for channel0."]
    #[inline(always)]
    pub fn rmt_rx_filter_en_ch0(&self) -> RMT_RX_FILTER_EN_CH0_R {
        RMT_RX_FILTER_EN_CH0_R::new(((self.bits >> 7) & 0x01) != 0)
    }
    #[doc = "Bit 6 - Set this bit to continue sending from the first data to the last data in channel0 again and again."]
    #[inline(always)]
    pub fn rmt_tx_conti_mode_ch0(&self) -> RMT_TX_CONTI_MODE_CH0_R {
        RMT_TX_CONTI_MODE_CH0_R::new(((self.bits >> 6) & 0x01) != 0)
    }
    #[doc = "Bit 5 - This is the mark of channel0's ram usage right.1'b1\u{ff1a}receiver uses the ram 0\u{ff1a}transmitter uses the ram"]
    #[inline(always)]
    pub fn rmt_mem_owner_ch0(&self) -> RMT_MEM_OWNER_CH0_R {
        RMT_MEM_OWNER_CH0_R::new(((self.bits >> 5) & 0x01) != 0)
    }
    #[doc = "Bit 4 - Set this bit to reset W/R ram address for channel0 by apb fifo access"]
    #[inline(always)]
    pub fn rmt_apb_mem_rst_ch0(&self) -> RMT_APB_MEM_RST_CH0_R {
        RMT_APB_MEM_RST_CH0_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - Set this bit to reset read ram address for channel0 by transmitter access."]
    #[inline(always)]
    pub fn rmt_mem_rd_rst_ch0(&self) -> RMT_MEM_RD_RST_CH0_R {
        RMT_MEM_RD_RST_CH0_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bit 2 - Set this bit to reset write ram address for channel0 by receiver access."]
    #[inline(always)]
    pub fn rmt_mem_wr_rst_ch0(&self) -> RMT_MEM_WR_RST_CH0_R {
        RMT_MEM_WR_RST_CH0_R::new(((self.bits >> 2) & 0x01) != 0)
    }
    #[doc = "Bit 1 - Set this bit to enbale receving data for channel0."]
    #[inline(always)]
    pub fn rmt_rx_en_ch0(&self) -> RMT_RX_EN_CH0_R {
        RMT_RX_EN_CH0_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set this bit to start sending data for channel0."]
    #[inline(always)]
    pub fn rmt_tx_start_ch0(&self) -> RMT_TX_START_CH0_R {
        RMT_TX_START_CH0_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 19 - This is the output enable control bit for channel0 in IDLE state."]
    #[inline(always)]
    pub fn rmt_idle_out_en_ch0(&mut self) -> RMT_IDLE_OUT_EN_CH0_W {
        RMT_IDLE_OUT_EN_CH0_W { w: self }
    }
    #[doc = "Bit 18 - This bit configures the output signal's level for channel0 in IDLE state."]
    #[inline(always)]
    pub fn rmt_idle_out_lv_ch0(&mut self) -> RMT_IDLE_OUT_LV_CH0_W {
        RMT_IDLE_OUT_LV_CH0_W { w: self }
    }
    #[doc = "Bit 17 - This bit is used to select base clock. 1'b1:clk_apb 1'b0:clk_ref"]
    #[inline(always)]
    pub fn rmt_ref_always_on_ch0(&mut self) -> RMT_REF_ALWAYS_ON_CH0_W {
        RMT_REF_ALWAYS_ON_CH0_W { w: self }
    }
    #[doc = "Bit 16 - This bit is used to reset divider in channel0."]
    #[inline(always)]
    pub fn rmt_ref_cnt_rst_ch0(&mut self) -> RMT_REF_CNT_RST_CH0_W {
        RMT_REF_CNT_RST_CH0_W { w: self }
    }
    #[doc = "Bits 8:15 - in receive mode channel0 ignore input pulse when the pulse width is smaller then this value."]
    #[inline(always)]
    pub fn rmt_rx_filter_thres_ch0(&mut self) -> RMT_RX_FILTER_THRES_CH0_W {
        RMT_RX_FILTER_THRES_CH0_W { w: self }
    }
    #[doc = "Bit 7 - This is the receive filter enable bit for channel0."]
    #[inline(always)]
    pub fn rmt_rx_filter_en_ch0(&mut self) -> RMT_RX_FILTER_EN_CH0_W {
        RMT_RX_FILTER_EN_CH0_W { w: self }
    }
    #[doc = "Bit 6 - Set this bit to continue sending from the first data to the last data in channel0 again and again."]
    #[inline(always)]
    pub fn rmt_tx_conti_mode_ch0(&mut self) -> RMT_TX_CONTI_MODE_CH0_W {
        RMT_TX_CONTI_MODE_CH0_W { w: self }
    }
    #[doc = "Bit 5 - This is the mark of channel0's ram usage right.1'b1\u{ff1a}receiver uses the ram 0\u{ff1a}transmitter uses the ram"]
    #[inline(always)]
    pub fn rmt_mem_owner_ch0(&mut self) -> RMT_MEM_OWNER_CH0_W {
        RMT_MEM_OWNER_CH0_W { w: self }
    }
    #[doc = "Bit 4 - Set this bit to reset W/R ram address for channel0 by apb fifo access"]
    #[inline(always)]
    pub fn rmt_apb_mem_rst_ch0(&mut self) -> RMT_APB_MEM_RST_CH0_W {
        RMT_APB_MEM_RST_CH0_W { w: self }
    }
    #[doc = "Bit 3 - Set this bit to reset read ram address for channel0 by transmitter access."]
    #[inline(always)]
    pub fn rmt_mem_rd_rst_ch0(&mut self) -> RMT_MEM_RD_RST_CH0_W {
        RMT_MEM_RD_RST_CH0_W { w: self }
    }
    #[doc = "Bit 2 - Set this bit to reset write ram address for channel0 by receiver access."]
    #[inline(always)]
    pub fn rmt_mem_wr_rst_ch0(&mut self) -> RMT_MEM_WR_RST_CH0_W {
        RMT_MEM_WR_RST_CH0_W { w: self }
    }
    #[doc = "Bit 1 - Set this bit to enbale receving data for channel0."]
    #[inline(always)]
    pub fn rmt_rx_en_ch0(&mut self) -> RMT_RX_EN_CH0_W {
        RMT_RX_EN_CH0_W { w: self }
    }
    #[doc = "Bit 0 - Set this bit to start sending data for channel0."]
    #[inline(always)]
    pub fn rmt_tx_start_ch0(&mut self) -> RMT_TX_START_CH0_W {
        RMT_TX_START_CH0_W { w: self }
    }
}
