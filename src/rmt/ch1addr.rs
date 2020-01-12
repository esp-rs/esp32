#[doc = "Reader of register CH1ADDR"]
pub type R = crate::R<u32, super::CH1ADDR>;
#[doc = "Writer for register CH1ADDR"]
pub type W = crate::W<u32, super::CH1ADDR>;
#[doc = "Register CH1ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH1ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_MEM_ADDR_CH1`"]
pub type APB_MEM_ADDR_CH1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APB_MEM_ADDR_CH1`"]
pub struct APB_MEM_ADDR_CH1_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_MEM_ADDR_CH1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The ram relative address in channel1 by apb fifo access"]
    #[inline(always)]
    pub fn apb_mem_addr_ch1(&self) -> APB_MEM_ADDR_CH1_R {
        APB_MEM_ADDR_CH1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The ram relative address in channel1 by apb fifo access"]
    #[inline(always)]
    pub fn apb_mem_addr_ch1(&mut self) -> APB_MEM_ADDR_CH1_W {
        APB_MEM_ADDR_CH1_W { w: self }
    }
}
