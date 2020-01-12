#[doc = "Reader of register PRO_BT_BB_INT_MAP"]
pub type R = crate::R<u32, super::PRO_BT_BB_INT_MAP>;
#[doc = "Writer for register PRO_BT_BB_INT_MAP"]
pub type W = crate::W<u32, super::PRO_BT_BB_INT_MAP>;
#[doc = "Register PRO_BT_BB_INT_MAP `reset()`'s with value 0"]
impl crate::ResetValue for super::PRO_BT_BB_INT_MAP {
    type Type = u32;
    #[inline(always)]
    fn reset_value() -> Self::Type {
        0
    }
}
#[doc = "Reader of field `PRO_BT_BB_INT_MAP`"]
pub type PRO_BT_BB_INT_MAP_R = crate::R<u8, u8>;
#[doc = "Write proxy for field `PRO_BT_BB_INT_MAP`"]
pub struct PRO_BT_BB_INT_MAP_W<'a> {
    w: &'a mut W,
}
impl<'a> PRO_BT_BB_INT_MAP_W<'a> {
    #[doc = r"Writes raw bits to the field"]
    #[inline(always)]
    pub unsafe fn bits(self, value: u8) -> &'a mut W {
        self.w.bits = (self.w.bits & !0x1f) | ((value as u32) & 0x1f);
        self.w
    }
}
impl R {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn pro_bt_bb_int_map(&self) -> PRO_BT_BB_INT_MAP_R {
        PRO_BT_BB_INT_MAP_R::new((self.bits & 0x1f) as u8)
    }
}
impl W {
    #[doc = "Bits 0:4"]
    #[inline(always)]
    pub fn pro_bt_bb_int_map(&mut self) -> PRO_BT_BB_INT_MAP_W {
        PRO_BT_BB_INT_MAP_W { w: self }
    }
}