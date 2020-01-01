#[doc = "Reader of register TRACEMEM_MUX_MODE"]
pub type R = crate::R<u32, super::TRACEMEM_MUX_MODE>;
#[doc = "Writer for register TRACEMEM_MUX_MODE"]
pub type W = crate::W<u32, super::TRACEMEM_MUX_MODE>;
#[doc = "Register TRACEMEM_MUX_MODE `reset()`'s with value 0"]
impl crate::ResetValue for super::TRACEMEM_MUX_MODE {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `DPORT_TRACEMEM_MUX_MODE`"]
pub type DPORT_TRACEMEM_MUX_MODE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `DPORT_TRACEMEM_MUX_MODE`"]
pub struct DPORT_TRACEMEM_MUX_MODE_W<'a> {
    w: &'a mut W,
}
impl<'a> DPORT_TRACEMEM_MUX_MODE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x03) | ((value as u32) & 0x03);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dport_tracemem_mux_mode(&self) -> DPORT_TRACEMEM_MUX_MODE_R {
        DPORT_TRACEMEM_MUX_MODE_R::new((self.bits & 0x03) as u8)
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn dport_tracemem_mux_mode(&mut self) -> DPORT_TRACEMEM_MUX_MODE_W {
        DPORT_TRACEMEM_MUX_MODE_W { w: self }
    }
}
