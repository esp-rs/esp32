#[doc = "Reader of register RMT_CH0CONF0_REG"]
pub type R = crate::R<u32, super::RMT_CH0CONF0_REG>;
#[doc = "Writer for register RMT_CH0CONF0_REG"]
pub type W = crate::W<u32, super::RMT_CH0CONF0_REG>;
#[doc = "Register RMT_CH0CONF0_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RMT_CH0CONF0_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMT_CLK_EN`"]
pub type RMT_CLK_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CLK_EN`"]
pub struct RMT_CLK_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CLK_EN_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 31)) | (((value as u32) & 0x01) << 31);
        self.w
    }
}
#[doc = "Reader of field `RMT_MEM_PD`"]
pub type RMT_MEM_PD_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_MEM_PD`"]
pub struct RMT_MEM_PD_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_PD_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 30)) | (((value as u32) & 0x01) << 30);
        self.w
    }
}
#[doc = "Reader of field `RMT_CARRIER_OUT_LV_CH0`"]
pub type RMT_CARRIER_OUT_LV_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CARRIER_OUT_LV_CH0`"]
pub struct RMT_CARRIER_OUT_LV_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CARRIER_OUT_LV_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 29)) | (((value as u32) & 0x01) << 29);
        self.w
    }
}
#[doc = "Reader of field `RMT_CARRIER_EN_CH0`"]
pub type RMT_CARRIER_EN_CH0_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CARRIER_EN_CH0`"]
pub struct RMT_CARRIER_EN_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CARRIER_EN_CH0_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 28)) | (((value as u32) & 0x01) << 28);
        self.w
    }
}
#[doc = "Reader of field `RMT_MEM_SIZE_CH0`"]
pub type RMT_MEM_SIZE_CH0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMT_MEM_SIZE_CH0`"]
pub struct RMT_MEM_SIZE_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_SIZE_CH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RMT_IDLE_THRES_CH0`"]
pub type RMT_IDLE_THRES_CH0_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RMT_IDLE_THRES_CH0`"]
pub struct RMT_IDLE_THRES_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_IDLE_THRES_CH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RMT_DIV_CNT_CH0`"]
pub type RMT_DIV_CNT_CH0_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMT_DIV_CNT_CH0`"]
pub struct RMT_DIV_CNT_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_DIV_CNT_CH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 31 - This bit is used to control clock.when software config RMT internal registers it controls the register clock."]
    #[inline(always)]
    pub fn rmt_clk_en(&self) -> RMT_CLK_EN_R {
        RMT_CLK_EN_R::new(((self.bits >> 31) & 0x01) != 0)
    }
    #[doc = "Bit 30 - This bit is used to reduce power consumed by mem. 1:mem is in low power state."]
    #[inline(always)]
    pub fn rmt_mem_pd(&self) -> RMT_MEM_PD_R {
        RMT_MEM_PD_R::new(((self.bits >> 30) & 0x01) != 0)
    }
    #[doc = "Bit 29 - This bit is used to configure the way carrier wave is modulated for channel0.1'b1:transmit on low output level 1'b0:transmit on high output level."]
    #[inline(always)]
    pub fn rmt_carrier_out_lv_ch0(&self) -> RMT_CARRIER_OUT_LV_CH0_R {
        RMT_CARRIER_OUT_LV_CH0_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable control bit for channel0."]
    #[inline(always)]
    pub fn rmt_carrier_en_ch0(&self) -> RMT_CARRIER_EN_CH0_R {
        RMT_CARRIER_EN_CH0_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - This register is used to configure the the amount of memory blocks allocated to channel0."]
    #[inline(always)]
    pub fn rmt_mem_size_ch0(&self) -> RMT_MEM_SIZE_CH0_R {
        RMT_MEM_SIZE_CH0_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - In receive mode when no edge is detected on the input signal for longer than reg_idle_thres_ch0 then the receive process is done."]
    #[inline(always)]
    pub fn rmt_idle_thres_ch0(&self) -> RMT_IDLE_THRES_CH0_R {
        RMT_IDLE_THRES_CH0_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 0:7 - This register is used to configure the frequency divider's factor in channel0."]
    #[inline(always)]
    pub fn rmt_div_cnt_ch0(&self) -> RMT_DIV_CNT_CH0_R {
        RMT_DIV_CNT_CH0_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 31 - This bit is used to control clock.when software config RMT internal registers it controls the register clock."]
    #[inline(always)]
    pub fn rmt_clk_en(&mut self) -> RMT_CLK_EN_W {
        RMT_CLK_EN_W { w: self }
    }
    #[doc = "Bit 30 - This bit is used to reduce power consumed by mem. 1:mem is in low power state."]
    #[inline(always)]
    pub fn rmt_mem_pd(&mut self) -> RMT_MEM_PD_W {
        RMT_MEM_PD_W { w: self }
    }
    #[doc = "Bit 29 - This bit is used to configure the way carrier wave is modulated for channel0.1'b1:transmit on low output level 1'b0:transmit on high output level."]
    #[inline(always)]
    pub fn rmt_carrier_out_lv_ch0(&mut self) -> RMT_CARRIER_OUT_LV_CH0_W {
        RMT_CARRIER_OUT_LV_CH0_W { w: self }
    }
    #[doc = "Bit 28 - This is the carrier modulation enable control bit for channel0."]
    #[inline(always)]
    pub fn rmt_carrier_en_ch0(&mut self) -> RMT_CARRIER_EN_CH0_W {
        RMT_CARRIER_EN_CH0_W { w: self }
    }
    #[doc = "Bits 24:27 - This register is used to configure the the amount of memory blocks allocated to channel0."]
    #[inline(always)]
    pub fn rmt_mem_size_ch0(&mut self) -> RMT_MEM_SIZE_CH0_W {
        RMT_MEM_SIZE_CH0_W { w: self }
    }
    #[doc = "Bits 8:23 - In receive mode when no edge is detected on the input signal for longer than reg_idle_thres_ch0 then the receive process is done."]
    #[inline(always)]
    pub fn rmt_idle_thres_ch0(&mut self) -> RMT_IDLE_THRES_CH0_W {
        RMT_IDLE_THRES_CH0_W { w: self }
    }
    #[doc = "Bits 0:7 - This register is used to configure the frequency divider's factor in channel0."]
    #[inline(always)]
    pub fn rmt_div_cnt_ch0(&mut self) -> RMT_DIV_CNT_CH0_W {
        RMT_DIV_CNT_CH0_W { w: self }
    }
}
