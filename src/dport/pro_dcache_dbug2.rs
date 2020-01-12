#[doc = "Reader of register PRO_DCACHE_DBUG2"]
pub type R = crate::R<u32, super::PRO_DCACHE_DBUG2>;
#[doc = "Writer for register PRO_DCACHE_DBUG2"]
pub type W = crate::W<u32, super::PRO_DCACHE_DBUG2>;
#[doc = "Register PRO_DCACHE_DBUG2 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_DCACHE_DBUG2 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRO_CACHE_VADDR`"]
pub type PRO_CACHE_VADDR_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `PRO_CACHE_VADDR`"]
pub struct PRO_CACHE_VADDR_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_CACHE_VADDR_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x07ff_ffff) | ((value as u32) & 0x07ff_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:26"]
    #[inline(always)]
    pub fn pro_cache_vaddr(&self) -> PRO_CACHE_VADDR_R {
        PRO_CACHE_VADDR_R::new((self.bits & 0x07ff_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:26"]
    #[inline(always)]
    pub fn pro_cache_vaddr(&mut self) -> PRO_CACHE_VADDR_W {
        PRO_CACHE_VADDR_W { w: self }
    }
}