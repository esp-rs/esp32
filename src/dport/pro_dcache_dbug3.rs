#[doc = "Reader of register PRO_DCACHE_DBUG3"]
pub type R = crate::R<u32, super::PRO_DCACHE_DBUG3>;
#[doc = "Writer for register PRO_DCACHE_DBUG3"]
pub type W = crate::W<u32, super::PRO_DCACHE_DBUG3>;
#[doc = "Register PRO_DCACHE_DBUG3 `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_DCACHE_DBUG3 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_PRO_CACHE_IRAM0_PID_ERROR`"]
pub type DPORT_PRO_CACHE_IRAM0_PID_ERROR_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `DPORT_PRO_CACHE_IRAM0_PID_ERROR`"]
pub struct DPORT_PRO_CACHE_IRAM0_PID_ERROR_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_CACHE_IRAM0_PID_ERROR_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 15)) | (((value as u32) & 0x01) << 15);
        self.w
    }
}
#[doc = "Reader of field `DPORT_PRO_CPU_DISABLED_CACHE_IA`"]
pub type DPORT_PRO_CPU_DISABLED_CACHE_IA_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_PRO_CPU_DISABLED_CACHE_IA`"]
pub struct DPORT_PRO_CPU_DISABLED_CACHE_IA_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_CPU_DISABLED_CACHE_IA_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x3f << 9)) | (((value as u32) & 0x3f) << 9);
        self.w
    }
}
impl R {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dport_pro_cache_iram0_pid_error(&self) -> DPORT_PRO_CACHE_IRAM0_PID_ERROR_R {
        DPORT_PRO_CACHE_IRAM0_PID_ERROR_R::new(((self.bits >> 15) & 0x01) != 0)
    }
    #[doc = "Bits 9:14"]
    #[inline(always)]
    pub fn dport_pro_cpu_disabled_cache_ia(&self) -> DPORT_PRO_CPU_DISABLED_CACHE_IA_R {
        DPORT_PRO_CPU_DISABLED_CACHE_IA_R::new(((self.bits >> 9) & 0x3f) as u8)
    }
}
impl W {
    #[doc = "Bit 15"]
    #[inline(always)]
    pub fn dport_pro_cache_iram0_pid_error(&mut self) -> DPORT_PRO_CACHE_IRAM0_PID_ERROR_W {
        DPORT_PRO_CACHE_IRAM0_PID_ERROR_W { w: self }
    }
    #[doc = "Bits 9:14"]
    #[inline(always)]
    pub fn dport_pro_cpu_disabled_cache_ia(&mut self) -> DPORT_PRO_CPU_DISABLED_CACHE_IA_W {
        DPORT_PRO_CPU_DISABLED_CACHE_IA_W { w: self }
    }
}
