#[doc = "Reader of register DPORT_PRO_DCACHE_DBUG4_REG"]
pub type R = crate::R<u32, super::DPORT_PRO_DCACHE_DBUG4_REG>;
#[doc = "Writer for register DPORT_PRO_DCACHE_DBUG4_REG"]
pub type W = crate::W<u32, super::DPORT_PRO_DCACHE_DBUG4_REG>;
#[doc = "Register DPORT_PRO_DCACHE_DBUG4_REG `reset()`'s with value 0"]
impl crate::ResetValue for super::DPORT_PRO_DCACHE_DBUG4_REG {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_PRO_DRAM1ADDR0_IA`"]
pub type DPORT_PRO_DRAM1ADDR0_IA_R = crate::R<u32, u32>;
#[doc = "Write proxy for field `DPORT_PRO_DRAM1ADDR0_IA`"]
pub struct DPORT_PRO_DRAM1ADDR0_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_DRAM1ADDR0_IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u32) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x000f_ffff) | ((value as u32) & 0x000f_ffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn dport_pro_dram1addr0_ia(&self) -> DPORT_PRO_DRAM1ADDR0_IA_R {
        DPORT_PRO_DRAM1ADDR0_IA_R::new((self.bits & 0x000f_ffff) as u32)
    }
}
impl W {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn dport_pro_dram1addr0_ia(&mut self) -> DPORT_PRO_DRAM1ADDR0_IA_W {
        DPORT_PRO_DRAM1ADDR0_IA_W { w: self }
    }
}
