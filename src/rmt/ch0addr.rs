#[doc = "Reader of register CH0ADDR"]
pub type R = crate::R<u32, super::CH0ADDR>;
#[doc = "Writer for register CH0ADDR"]
pub type W = crate::W<u32, super::CH0ADDR>;
#[doc = "Register CH0ADDR `reset()`'s with value 0"]
impl crate::ResetValue for super::CH0ADDR {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `APB_MEM_ADDR_CH0`"]
pub type APB_MEM_ADDR_CH0_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `APB_MEM_ADDR_CH0`"]
pub struct APB_MEM_ADDR_CH0_W<'a> {
    w: &'a mut W,
}
impl<'a> APB_MEM_ADDR_CH0_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31 - The ram relative address in channel0 by apb fifo access"]
    #[inline(always)]
    pub fn apb_mem_addr_ch0(&self) -> APB_MEM_ADDR_CH0_R {
        APB_MEM_ADDR_CH0_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31 - The ram relative address in channel0 by apb fifo access"]
    #[inline(always)]
    pub fn apb_mem_addr_ch0(&mut self) -> APB_MEM_ADDR_CH0_W {
        APB_MEM_ADDR_CH0_W { w: self }
    }
}
