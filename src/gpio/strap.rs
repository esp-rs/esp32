#[doc = "Reader of register STRAP"]
pub type R = crate::R<u32, super::STRAP>;
#[doc = "Writer for register STRAP"]
pub type W = crate::W<u32, super::STRAP>;
#[doc = "Register STRAP `reset()`'s with value 0"]
impl crate::ResetValue for super::STRAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `STRAPPING`"]
pub type STRAPPING_R = crate::R<u16, u16>;
#[doc = "Write proxy for field `STRAPPING`"]
pub struct STRAPPING_W<'a> {
    w: &'a mut W,
}
impl<'a> STRAPPING_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u16) -> &'a mut W {
        self.w.bits = (self.w.bits & !0xffff) | ((value as u32) & 0xffff);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:15 - {10'b0, MTDI, GPIO0, GPIO2, GPIO4, MTDO, GPIO5}"]
    #[inline(always)]
    pub fn strapping(&self) -> STRAPPING_R {
        STRAPPING_R::new((self.bits & 0xffff) as u16)
    }
}
impl W {
    #[doc = "Bits 0:15 - {10'b0, MTDI, GPIO0, GPIO2, GPIO4, MTDO, GPIO5}"]
    #[inline(always)]
    pub fn strapping(&mut self) -> STRAPPING_W {
        STRAPPING_W { w: self }
    }
}
