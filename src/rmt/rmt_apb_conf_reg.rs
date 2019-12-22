#[doc = "Reader of register RMT_APB_CONF_REG"]
pub type R = crate::R<u32, super::RMT_APB_CONF_REG>;
#[doc = "Writer for register RMT_APB_CONF_REG"]
pub type W = crate::W<u32, super::RMT_APB_CONF_REG>;
#[doc = "Register RMT_APB_CONF_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::RMT_APB_CONF_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `RMT_MEM_TX_WRAP_EN`"]
pub type RMT_MEM_TX_WRAP_EN_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_MEM_TX_WRAP_EN`"]
pub struct RMT_MEM_TX_WRAP_EN_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_MEM_TX_WRAP_EN_W<'a> {
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
#[doc = "Reader of field `RMT_APB_FIFO_MASK`"]
pub type RMT_APB_FIFO_MASK_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `RMT_APB_FIFO_MASK`"]
pub struct RMT_APB_FIFO_MASK_W<'a> {
    w: &'a mut W,
}
impl<'a> RMT_APB_FIFO_MASK_W<'a> {
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
    #[doc = "Bit 1 - when datas need to be send is more than channel's mem can store then set this bit to enable reusage of mem this bit is used together with reg_rmt_tx_lim_chn."]
    #[inline(always)]
    pub fn rmt_mem_tx_wrap_en(&self) -> RMT_MEM_TX_WRAP_EN_R {
        RMT_MEM_TX_WRAP_EN_R::new(((self.bits >> 1) & 0x01) != 0)
    }
    #[doc = "Bit 0 - Set this bit to disable apb fifo access"]
    #[inline(always)]
    pub fn rmt_apb_fifo_mask(&self) -> RMT_APB_FIFO_MASK_R {
        RMT_APB_FIFO_MASK_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 1 - when datas need to be send is more than channel's mem can store then set this bit to enable reusage of mem this bit is used together with reg_rmt_tx_lim_chn."]
    #[inline(always)]
    pub fn rmt_mem_tx_wrap_en(&mut self) -> RMT_MEM_TX_WRAP_EN_W {
        RMT_MEM_TX_WRAP_EN_W { w: self }
    }
    #[doc = "Bit 0 - Set this bit to disable apb fifo access"]
    #[inline(always)]
    pub fn rmt_apb_fifo_mask(&mut self) -> RMT_APB_FIFO_MASK_W {
        RMT_APB_FIFO_MASK_W { w: self }
    }
}
