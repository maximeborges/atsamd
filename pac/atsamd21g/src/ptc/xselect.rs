#[doc = "Register `XSELECT` reader"]
pub type R = crate::R<XselectSpec>;
#[doc = "Register `XSELECT` writer"]
pub type W = crate::W<XselectSpec>;
#[doc = "X line selection MUX\n\nValue on reset: 0"]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u16)]
pub enum Xmuxselect {
    #[doc = "1: PTC X0 Pin"]
    X0 = 1,
    #[doc = "2: PTC X1 Pin"]
    X1 = 2,
    #[doc = "4: PTC X2 Pin"]
    X2 = 4,
    #[doc = "8: PTC X3 Pin"]
    X3 = 8,
    #[doc = "16: PTC X4 Pin"]
    X4 = 16,
    #[doc = "32: PTC X5 Pin"]
    X5 = 32,
    #[doc = "64: PTC X6 Pin"]
    X6 = 64,
    #[doc = "128: PTC X7 Pin"]
    X7 = 128,
    #[doc = "256: PTC X8 Pin"]
    X8 = 256,
    #[doc = "512: PTC X9 Pin"]
    X9 = 512,
    #[doc = "1024: PTC X10 Pin"]
    X10 = 1024,
    #[doc = "2048: PTC X11 Pin"]
    X11 = 2048,
    #[doc = "4096: PTC X12 Pin"]
    X12 = 4096,
    #[doc = "8192: PTC X13 Pin"]
    X13 = 8192,
    #[doc = "16384: PTC X14 Pin"]
    X14 = 16384,
    #[doc = "32768: PTC X15 Pin"]
    X15 = 32768,
}
impl From<Xmuxselect> for u16 {
    #[inline(always)]
    fn from(variant: Xmuxselect) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for Xmuxselect {
    type Ux = u16;
}
impl crate::IsEnum for Xmuxselect {}
#[doc = "Field `XMUX` reader - X line selection MUX"]
pub type XmuxR = crate::FieldReader<Xmuxselect>;
impl XmuxR {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> Option<Xmuxselect> {
        match self.bits {
            1 => Some(Xmuxselect::X0),
            2 => Some(Xmuxselect::X1),
            4 => Some(Xmuxselect::X2),
            8 => Some(Xmuxselect::X3),
            16 => Some(Xmuxselect::X4),
            32 => Some(Xmuxselect::X5),
            64 => Some(Xmuxselect::X6),
            128 => Some(Xmuxselect::X7),
            256 => Some(Xmuxselect::X8),
            512 => Some(Xmuxselect::X9),
            1024 => Some(Xmuxselect::X10),
            2048 => Some(Xmuxselect::X11),
            4096 => Some(Xmuxselect::X12),
            8192 => Some(Xmuxselect::X13),
            16384 => Some(Xmuxselect::X14),
            32768 => Some(Xmuxselect::X15),
            _ => None,
        }
    }
    #[doc = "PTC X0 Pin"]
    #[inline(always)]
    pub fn is_x0(&self) -> bool {
        *self == Xmuxselect::X0
    }
    #[doc = "PTC X1 Pin"]
    #[inline(always)]
    pub fn is_x1(&self) -> bool {
        *self == Xmuxselect::X1
    }
    #[doc = "PTC X2 Pin"]
    #[inline(always)]
    pub fn is_x2(&self) -> bool {
        *self == Xmuxselect::X2
    }
    #[doc = "PTC X3 Pin"]
    #[inline(always)]
    pub fn is_x3(&self) -> bool {
        *self == Xmuxselect::X3
    }
    #[doc = "PTC X4 Pin"]
    #[inline(always)]
    pub fn is_x4(&self) -> bool {
        *self == Xmuxselect::X4
    }
    #[doc = "PTC X5 Pin"]
    #[inline(always)]
    pub fn is_x5(&self) -> bool {
        *self == Xmuxselect::X5
    }
    #[doc = "PTC X6 Pin"]
    #[inline(always)]
    pub fn is_x6(&self) -> bool {
        *self == Xmuxselect::X6
    }
    #[doc = "PTC X7 Pin"]
    #[inline(always)]
    pub fn is_x7(&self) -> bool {
        *self == Xmuxselect::X7
    }
    #[doc = "PTC X8 Pin"]
    #[inline(always)]
    pub fn is_x8(&self) -> bool {
        *self == Xmuxselect::X8
    }
    #[doc = "PTC X9 Pin"]
    #[inline(always)]
    pub fn is_x9(&self) -> bool {
        *self == Xmuxselect::X9
    }
    #[doc = "PTC X10 Pin"]
    #[inline(always)]
    pub fn is_x10(&self) -> bool {
        *self == Xmuxselect::X10
    }
    #[doc = "PTC X11 Pin"]
    #[inline(always)]
    pub fn is_x11(&self) -> bool {
        *self == Xmuxselect::X11
    }
    #[doc = "PTC X12 Pin"]
    #[inline(always)]
    pub fn is_x12(&self) -> bool {
        *self == Xmuxselect::X12
    }
    #[doc = "PTC X13 Pin"]
    #[inline(always)]
    pub fn is_x13(&self) -> bool {
        *self == Xmuxselect::X13
    }
    #[doc = "PTC X14 Pin"]
    #[inline(always)]
    pub fn is_x14(&self) -> bool {
        *self == Xmuxselect::X14
    }
    #[doc = "PTC X15 Pin"]
    #[inline(always)]
    pub fn is_x15(&self) -> bool {
        *self == Xmuxselect::X15
    }
}
#[doc = "Field `XMUX` writer - X line selection MUX"]
pub type XmuxW<'a, REG> = crate::FieldWriter<'a, REG, 16, Xmuxselect>;
impl<'a, REG> XmuxW<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u16>,
{
    #[doc = "PTC X0 Pin"]
    #[inline(always)]
    pub fn x0(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X0)
    }
    #[doc = "PTC X1 Pin"]
    #[inline(always)]
    pub fn x1(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X1)
    }
    #[doc = "PTC X2 Pin"]
    #[inline(always)]
    pub fn x2(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X2)
    }
    #[doc = "PTC X3 Pin"]
    #[inline(always)]
    pub fn x3(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X3)
    }
    #[doc = "PTC X4 Pin"]
    #[inline(always)]
    pub fn x4(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X4)
    }
    #[doc = "PTC X5 Pin"]
    #[inline(always)]
    pub fn x5(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X5)
    }
    #[doc = "PTC X6 Pin"]
    #[inline(always)]
    pub fn x6(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X6)
    }
    #[doc = "PTC X7 Pin"]
    #[inline(always)]
    pub fn x7(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X7)
    }
    #[doc = "PTC X8 Pin"]
    #[inline(always)]
    pub fn x8(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X8)
    }
    #[doc = "PTC X9 Pin"]
    #[inline(always)]
    pub fn x9(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X9)
    }
    #[doc = "PTC X10 Pin"]
    #[inline(always)]
    pub fn x10(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X10)
    }
    #[doc = "PTC X11 Pin"]
    #[inline(always)]
    pub fn x11(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X11)
    }
    #[doc = "PTC X12 Pin"]
    #[inline(always)]
    pub fn x12(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X12)
    }
    #[doc = "PTC X13 Pin"]
    #[inline(always)]
    pub fn x13(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X13)
    }
    #[doc = "PTC X14 Pin"]
    #[inline(always)]
    pub fn x14(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X14)
    }
    #[doc = "PTC X15 Pin"]
    #[inline(always)]
    pub fn x15(self) -> &'a mut crate::W<REG> {
        self.variant(Xmuxselect::X15)
    }
}
impl R {
    #[doc = "Bits 0:15 - X line selection MUX"]
    #[inline(always)]
    pub fn xmux(&self) -> XmuxR {
        XmuxR::new(self.bits)
    }
}
impl W {
    #[doc = "Bits 0:15 - X line selection MUX"]
    #[inline(always)]
    #[must_use]
    pub fn xmux(&mut self) -> XmuxW<XselectSpec> {
        XmuxW::new(self, 0)
    }
}
#[doc = "Select X line\n\nYou can [`read`](crate::Reg::read) this register and get [`xselect::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`xselect::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct XselectSpec;
impl crate::RegisterSpec for XselectSpec {
    type Ux = u16;
}
#[doc = "`read()` method returns [`xselect::R`](R) reader structure"]
impl crate::Readable for XselectSpec {}
#[doc = "`write(|w| ..)` method takes [`xselect::W`](W) writer structure"]
impl crate::Writable for XselectSpec {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u16 = 0;
}
#[doc = "`reset()` method sets XSELECT to value 0"]
impl crate::Resettable for XselectSpec {
    const RESET_VALUE: u16 = 0;
}
