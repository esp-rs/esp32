#[doc = "Reader of register PRO_INTRUSION_STATUS"]
pub type R = crate::R<u32, super::PRO_INTRUSION_STATUS>;
#[doc = "Writer for register PRO_INTRUSION_STATUS"]
pub type W = crate::W<u32, super::PRO_INTRUSION_STATUS>;
#[doc = "Register PRO_INTRUSION_STATUS `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_INTRUSION_STATUS {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_PRO_INTRUSION_RECORD`"]
pub type DPORT_PRO_INTRUSION_RECORD_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_PRO_INTRUSION_RECORD`"]
pub struct DPORT_PRO_INTRUSION_RECORD_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_PRO_INTRUSION_RECORD_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x0f) | ((value as u32) & 0x0f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dport_pro_intrusion_record(&self) -> DPORT_PRO_INTRUSION_RECORD_R {
        DPORT_PRO_INTRUSION_RECORD_R::new((self.bits & 0x0f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn dport_pro_intrusion_record(&mut self) -> DPORT_PRO_INTRUSION_RECORD_W {
        DPORT_PRO_INTRUSION_RECORD_W { w: self }
    }
}
