#[doc = "Reader of register PRO_DPORT_APB_MASK1"]
pub type R = crate::R<u32, super::PRO_DPORT_APB_MASK1>;
#[doc = "Writer for register PRO_DPORT_APB_MASK1"]
pub type W = crate::W<u32, super::PRO_DPORT_APB_MASK1>;
#[doc = "Register PRO_DPORT_APB_MASK1 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_DPORT_APB_MASK1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRODPORT_APB_MASK1`"]
pub type PRODPORT_APB_MASK1_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PRODPORT_APB_MASK1`"]
pub struct PRODPORT_APB_MASK1_W<'a> {
    w: &'a mut W,
}
impl<'a> PRODPORT_APB_MASK1_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff_ffff) | ((value as u32) & 0xffff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn prodport_apb_mask1(&self) -> PRODPORT_APB_MASK1_R {
        PRODPORT_APB_MASK1_R::new((self.bits & 0xffff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn prodport_apb_mask1(&mut self) -> PRODPORT_APB_MASK1_W {
        PRODPORT_APB_MASK1_W { w: self }
    }
}
