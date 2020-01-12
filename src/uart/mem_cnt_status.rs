#[doc = "Reader of register MEM_CNT_STATUS"]
pub type R = crate::R<u32, super::MEM_CNT_STATUS>;
#[doc = "Writer for register MEM_CNT_STATUS"]
pub type W = crate::W<u32, super::MEM_CNT_STATUS>;
#[doc = "Register MEM_CNT_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::MEM_CNT_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `TX_MEM_CNT`"]
pub type TX_MEM_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `TX_MEM_CNT`"]
pub struct TX_MEM_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> TX_MEM_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x07 << 3)) | (((value as u32) & 0x07) << 3);
        self.w
    }
}
#[doc = "Reader of field `RX_MEM_CNT`"]
pub type RX_MEM_CNT_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `RX_MEM_CNT`"]
pub struct RX_MEM_CNT_W<'a> {
    w: &'a mut W,
}
impl<'a> RX_MEM_CNT_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07) | ((value as u32) & 0x07);
        self.w
    }
}
impl R {
    #[doc = "Bits 3:5 - refer to the txfifo_cnt's describtion."]
    #[inline(always)]
    pub fn tx_mem_cnt(&self) -> TX_MEM_CNT_R {
        TX_MEM_CNT_R::new(((self.bits >> 3) & 0x07) as u8)
    }
    #[doc = "Bits 0:2 - refer to the rxfifo_cnt's describtion."]
    #[inline(always)]
    pub fn rx_mem_cnt(&self) -> RX_MEM_CNT_R {
        RX_MEM_CNT_R::new((self.bits & 0x07) as u8)
    }
}
impl W {
    #[doc = "Bits 3:5 - refer to the txfifo_cnt's describtion."]
    #[inline(always)]
    pub fn tx_mem_cnt(&mut self) -> TX_MEM_CNT_W {
        TX_MEM_CNT_W { w: self }
    }
    #[doc = "Bits 0:2 - refer to the rxfifo_cnt's describtion."]
    #[inline(always)]
    pub fn rx_mem_cnt(&mut self) -> RX_MEM_CNT_W {
        RX_MEM_CNT_W { w: self }
    }
}
