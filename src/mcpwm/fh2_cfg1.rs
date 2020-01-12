#[doc = "Reader of register FH2_CFG1"]
pub type R = crate::R<u32, super::FH2_CFG1>;
#[doc = "Writer for register FH2_CFG1"]
pub type W = crate::W<u32, super::FH2_CFG1>;
#[doc = "Register FH2_CFG1 `reset()`'s with value 0"]
impl crate::ResetValue for super::FH2_CFG1 {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `FH2_FORCE_OST`"]
pub type FH2_FORCE_OST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FH2_FORCE_OST`"]
pub struct FH2_FORCE_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_FORCE_OST_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 4)) | (((value as u32) & 0x01) << 4);
        self.w
    }
}
#[doc = "Reader of field `FH2_FORCE_CBC`"]
pub type FH2_FORCE_CBC_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FH2_FORCE_CBC`"]
pub struct FH2_FORCE_CBC_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_FORCE_CBC_W<'a> {
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
        self.w.bits = (self.w.bits & !(0x01 << 3)) | (((value as u32) & 0x01) << 3);
        self.w
    }
}
#[doc = "Reader of field `FH2_CBCPULSE`"]
pub type FH2_CBCPULSE_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `FH2_CBCPULSE`"]
pub struct FH2_CBCPULSE_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_CBCPULSE_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !(0x03 << 1)) | (((value as u32) & 0x03) << 1);
        self.w
    }
}
#[doc = "Reader of field `FH2_CLR_OST`"]
pub type FH2_CLR_OST_R = crate::R<bool, bool>;
#[doc = "Write proxy for field `FH2_CLR_OST`"]
pub struct FH2_CLR_OST_W<'a> {
    w: &'a mut W,
}
impl<'a> FH2_CLR_OST_W<'a> {
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
        self.w.bits = (self.w.bits & !0x01) | ((value as u32) & 0x01);
        self.w
    }
}
impl R {
    #[doc = "Bit 4 - A toggle (software negation of value of this bit) triggers a one-shot mode action"]
    #[inline(always)]
    pub fn fh2_force_ost(&self) -> FH2_FORCE_OST_R {
        FH2_FORCE_OST_R::new(((self.bits >> 4) & 0x01) != 0)
    }
    #[doc = "Bit 3 - A toggle triggers a cycle-by-cycle mode action"]
    #[inline(always)]
    pub fn fh2_force_cbc(&self) -> FH2_FORCE_CBC_R {
        FH2_FORCE_CBC_R::new(((self.bits >> 3) & 0x01) != 0)
    }
    #[doc = "Bits 1:2 - The cycle-by-cycle mode action refresh moment selection. Bit0: TEZ bit1:TEP"]
    #[inline(always)]
    pub fn fh2_cbcpulse(&self) -> FH2_CBCPULSE_R {
        FH2_CBCPULSE_R::new(((self.bits >> 1) & 0x03) as u8)
    }
    #[doc = "Bit 0 - A toggle will clear on going one-shot mode action"]
    #[inline(always)]
    pub fn fh2_clr_ost(&self) -> FH2_CLR_OST_R {
        FH2_CLR_OST_R::new((self.bits & 0x01) != 0)
    }
}
impl W {
    #[doc = "Bit 4 - A toggle (software negation of value of this bit) triggers a one-shot mode action"]
    #[inline(always)]
    pub fn fh2_force_ost(&mut self) -> FH2_FORCE_OST_W {
        FH2_FORCE_OST_W { w: self }
    }
    #[doc = "Bit 3 - A toggle triggers a cycle-by-cycle mode action"]
    #[inline(always)]
    pub fn fh2_force_cbc(&mut self) -> FH2_FORCE_CBC_W {
        FH2_FORCE_CBC_W { w: self }
    }
    #[doc = "Bits 1:2 - The cycle-by-cycle mode action refresh moment selection. Bit0: TEZ bit1:TEP"]
    #[inline(always)]
    pub fn fh2_cbcpulse(&mut self) -> FH2_CBCPULSE_W {
        FH2_CBCPULSE_W { w: self }
    }
    #[doc = "Bit 0 - A toggle will clear on going one-shot mode action"]
    #[inline(always)]
    pub fn fh2_clr_ost(&mut self) -> FH2_CLR_OST_W {
        FH2_CLR_OST_W { w: self }
    }
}
