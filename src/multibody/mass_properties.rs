/// Represents the mass properties of an object
/// Mass, Center of Mass, Inertia
#[derive(Debug, Clone, Copy)]
pub struct MassProperties {
    cmx: f64,
    cmy: f64,
    cmz: f64,
    mass: f64,
    ixx: f64,
    ixy: f64,
    ixz: f64,
    iyy: f64,
    iyz: f64,
    izz: f64,
}

/// Enum representing possible errors when creating or modifying `MassProperties`.
#[derive(Debug, Clone, Copy)]
pub enum MassPropertiesError {
    IxxLessThanOrEqualToZero,
    IyyLessThanOrEqualToZero,
    IzzLessThanOrEqualToZero,
    MassLessThanOrEqualToZero,
}

impl MassProperties {
    /// Creates a new `MassProperties` instance.
    ///
    /// # Arguments
    ///
    /// * `mass` - The mass of the object.
    /// * `cmx` - The x-coordinate of the center of mass.
    /// * `cmy` - The y-coordinate of the center of mass.
    /// * `cmz` - The z-coordinate of the center of mass.
    /// * `ixx` - The moment of inertia around the x-axis.
    /// * `iyy` - The moment of inertia around the y-axis.
    /// * `izz` - The moment of inertia around the z-axis.
    /// * `ixy` - The product of inertia for the xy-plane.
    /// * `ixz` - The product of inertia for the xz-plane.
    /// * `iyz` - The product of inertia for the yz-plane.
    ///
    /// # Errors
    ///
    /// Returns a `MassPropertiesError` if any of the mass or principal moments of inertia
    /// are less than or equal to zero.
    pub fn new(
        mass: f64,
        cmx: f64,
        cmy: f64,
        cmz: f64,
        ixx: f64,
        iyy: f64,
        izz: f64,
        ixy: f64,
        ixz: f64,
        iyz: f64,
    ) -> Result<Self, MassPropertiesError> {
        if mass <= 0.0 {
            return Err(MassPropertiesError::MassLessThanOrEqualToZero);
        }
        if ixx <= 0.0 {
            return Err(MassPropertiesError::IxxLessThanOrEqualToZero);
        }
        if iyy <= 0.0 {
            return Err(MassPropertiesError::IyyLessThanOrEqualToZero);
        }
        if izz <= 0.0 {
            return Err(MassPropertiesError::IzzLessThanOrEqualToZero);
        }

        Ok(Self {
            mass,
            cmx,
            cmy,
            cmz,
            ixx,
            iyy,
            izz,
            ixy,
            ixz,
            iyz,
        })
    }

    /// Returns the x-coordinate of the center of mass.
    pub fn get_cmx(&self) -> f64 {
        self.cmx
    }

    /// Returns the y-coordinate of the center of mass.
    pub fn get_cmy(&self) -> f64 {
        self.cmy
    }

    /// Returns the y-coordinate of the center of mass.
    pub fn get_cmz(&self) -> f64 {
        self.cmz
    }

    /// Returns the moment of inertia around the x-axis.
    pub fn get_ixx(&self) -> f64 {
        self.ixx
    }

    /// Returns the product of inertia for the xy-plane.
    pub fn get_ixy(&self) -> f64 {
        self.ixy
    }

    /// Returns the product of inertia for the xz-plane.
    pub fn get_ixz(&self) -> f64 {
        self.ixz
    }

    /// Returns the moment of inertia around the y-axis.
    pub fn get_iyy(&self) -> f64 {
        self.iyy
    }

    /// Returns the product of inertia for the yz-plane.
    pub fn get_iyz(&self) -> f64 {
        self.iyz
    }

    /// Returns the moment of inertia around the z-axis.
    pub fn get_izz(&self) -> f64 {
        self.izz
    }

    /// Returns the mass of the object.
    pub fn get_mass(&self) -> f64 {
        self.mass
    }

    /// Sets the x-coordinate of the center of mass.
    pub fn set_cmx(&mut self, cmx: f64) -> Result<(), MassPropertiesError> {
        self.cmx = cmx;
        Ok(())
    }

    /// Sets the y-coordinate of the center of mass.
    pub fn set_cmy(&mut self, cmy: f64) -> Result<(), MassPropertiesError> {
        self.cmy = cmy;
        Ok(())
    }

    /// Sets the z-coordinate of the center of mass.
    pub fn set_cmz(&mut self, cmz: f64) -> Result<(), MassPropertiesError> {
        self.cmz = cmz;
        Ok(())
    }

    /// Sets the moment of inertia around the x-axis.
    ///
    /// # Errors
    ///
    /// Returns a `MassPropertiesError::IxxLessThanOrEqualToZero` if `ixx` is less than or equal to zero.
    pub fn set_ixx(&mut self, ixx: f64) -> Result<(), MassPropertiesError> {
        if ixx <= 0.0 {
            return Err(MassPropertiesError::IxxLessThanOrEqualToZero);
        }
        self.ixx = ixx;
        Ok(())
    }

    /// Sets the product of inertia for the xy-plane.
    pub fn set_ixy(&mut self, ixy: f64) -> Result<(), MassPropertiesError> {
        self.ixy = ixy;
        Ok(())
    }

    /// Sets the product of inertia for the xz-plane.
    pub fn set_ixz(&mut self, ixz: f64) -> Result<(), MassPropertiesError> {
        self.ixz = ixz;
        Ok(())
    }

    /// Sets the moment of inertia around the y-axis.
    ///
    /// # Errors
    ///
    /// Returns a `MassPropertiesError::IyyLessThanOrEqualToZero` if `iyy` is less than or equal to zero.
    pub fn set_iyy(&mut self, iyy: f64) -> Result<(), MassPropertiesError> {
        if iyy <= 0.0 {
            return Err(MassPropertiesError::IyyLessThanOrEqualToZero);
        }
        self.iyy = iyy;
        Ok(())
    }

    /// Sets the product of inertia for the yz-plane.
    pub fn set_iyz(&mut self, iyz: f64) -> Result<(), MassPropertiesError> {
        self.iyz = iyz;
        Ok(())
    }

    /// Sets the moment of inertia around the z-axis.
    ///
    /// # Errors
    ///
    /// Returns a `MassPropertiesError::IzzLessThanOrEqualToZero` if `izz` is less than or equal to zero.
    pub fn set_izz(&mut self, izz: f64) -> Result<(), MassPropertiesError> {
        if izz <= 0.0 {
            return Err(MassPropertiesError::IzzLessThanOrEqualToZero);
        }
        self.izz = izz;
        Ok(())
    }

    /// Sets the mass of the object.
    ///
    /// # Errors
    ///
    /// Returns a `MassPropertiesError::MassLessThanOrEqualToZero` if `mass` is less than or equal to zero.
    pub fn set_mass(&mut self, mass: f64) -> Result<(), MassPropertiesError> {
        if mass <= 0.0 {
            return Err(MassPropertiesError::MassLessThanOrEqualToZero);
        }
        self.mass = mass;
        Ok(())
    }
}
