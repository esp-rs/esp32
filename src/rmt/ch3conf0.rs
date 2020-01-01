#[doc = "Reader of register CH3CONF0"]
pub type R = crate::R<u32, super::CH3CONF0>;
#[doc = "Writer for register CH3CONF0"]
pub type W = crate::W<u32, super::CH3CONF0>;
#[doc = "Register CH3CONF0 `reset()`'s with value 0"]
impl crate::ResetValue for super::CH3CONF0 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMT_CARRIER_OUT_LV_CH3`"]
pub type RMT_CARRIER_OUT_LV_CH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CARRIER_OUT_LV_CH3`"]
pub struct RMT_CARRIER_OUT_LV_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CARRIER_OUT_LV_CH3_W<'a> {
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
#[doc = "Reader of field `RMT_CARRIER_EN_CH3`"]
pub type RMT_CARRIER_EN_CH3_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_CARRIER_EN_CH3`"]
pub struct RMT_CARRIER_EN_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_CARRIER_EN_CH3_W<'a> {
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
#[doc = "Reader of field `RMT_MEM_SIZE_CH3`"]
pub type RMT_MEM_SIZE_CH3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMT_MEM_SIZE_CH3`"]
pub struct RMT_MEM_SIZE_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_SIZE_CH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x0f << 24)) | (((value as u32) & 0x0f) << 24);
        self.w
    }
}
#[doc = "Reader of field `RMT_IDLE_THRES_CH3`"]
pub type RMT_IDLE_THRES_CH3_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `RMT_IDLE_THRES_CH3`"]
pub struct RMT_IDLE_THRES_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_IDLE_THRES_CH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0xffff << 8)) | (((value as u32) & 0xffff) << 8);
        self.w
    }
}
#[doc = "Reader of field `RMT_DIV_CNT_CH3`"]
pub type RMT_DIV_CNT_CH3_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RMT_DIV_CNT_CH3`"]
pub struct RMT_DIV_CNT_CH3_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_DIV_CNT_CH3_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xff) | ((value as u32) & 0xff);
        self.w
    }
}
impl R {
    #[doc = "Bit 29 - This bit is used to configure carrier wave's position for channel3.1'b1:add on low level 1'b0:add on high level."]
    #[inline(always)]
    pub fn rmt_carrier_out_lv_ch3(&self) -> RMT_CARRIER_OUT_LV_CH3_R {
        RMT_CARRIER_OUT_LV_CH3_R::new(((self.bits >> 29) & 0x01) != 0)
    }
    #[doc = "Bit 28 - This is the carrier modulation enable control bit for channel3."]
    #[inline(always)]
    pub fn rmt_carrier_en_ch3(&self) -> RMT_CARRIER_EN_CH3_R {
        RMT_CARRIER_EN_CH3_R::new(((self.bits >> 28) & 0x01) != 0)
    }
    #[doc = "Bits 24:27 - This register is used to configure the the amount of memory blocks allocated to channel3."]
    #[inline(always)]
    pub fn rmt_mem_size_ch3(&self) -> RMT_MEM_SIZE_CH3_R {
        RMT_MEM_SIZE_CH3_R::new(((self.bits >> 24) & 0x0f) as u8)
    }
    #[doc = "Bits 8:23 - In receive mode when the counter's value is bigger than reg_idle_thres_ch3 then the receive process is done."]
    #[inline(always)]
    pub fn rmt_idle_thres_ch3(&self) -> RMT_IDLE_THRES_CH3_R {
        RMT_IDLE_THRES_CH3_R::new(((self.bits >> 8) & 0xffff) as u16)
    }
    #[doc = "Bits 0:7 - This register is used to configure the frequency divider's factor in channel3."]
    #[inline(always)]
    pub fn rmt_div_cnt_ch3(&self) -> RMT_DIV_CNT_CH3_R {
        RMT_DIV_CNT_CH3_R::new((self.bits & 0xff) as u8)
    }
}
impl W {
    #[doc = "Bit 29 - This bit is used to configure carrier wave's position for channel3.1'b1:add on low level 1'b0:add on high level."]
    #[inline(always)]
    pub fn rmt_carrier_out_lv_ch3(&mut self) -> RMT_CARRIER_OUT_LV_CH3_W {
        RMT_CARRIER_OUT_LV_CH3_W { w: self }
    }
    #[doc = "Bit 28 - This is the carrier modulation enable control bit for channel3."]
    #[inline(always)]
    pub fn rmt_carrier_en_ch3(&mut self) -> RMT_CARRIER_EN_CH3_W {
        RMT_CARRIER_EN_CH3_W { w: self }
    }
    #[doc = "Bits 24:27 - This register is used to configure the the amount of memory blocks allocated to channel3."]
    #[inline(always)]
    pub fn rmt_mem_size_ch3(&mut self) -> RMT_MEM_SIZE_CH3_W {
        RMT_MEM_SIZE_CH3_W { w: self }
    }
    #[doc = "Bits 8:23 - In receive mode when the counter's value is bigger than reg_idle_thres_ch3 then the receive process is done."]
    #[inline(always)]
    pub fn rmt_idle_thres_ch3(&mut self) -> RMT_IDLE_THRES_CH3_W {
        RMT_IDLE_THRES_CH3_W { w: self }
    }
    #[doc = "Bits 0:7 - This register is used to configure the frequency divider's factor in channel3."]
    #[inline(always)]
    pub fn rmt_div_cnt_ch3(&mut self) -> RMT_DIV_CNT_CH3_W {
        RMT_DIV_CNT_CH3_W { w: self }
    }
}
