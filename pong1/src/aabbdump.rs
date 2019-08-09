
type Rect = [f64;4];
type Vect = [f64;2];

/// checks if a given axis alligned bounding box "aabb" contains a point x,y
pub fn rectpointcoll(aabb: &Rect, x: &f64, y: &f64)->bool{
    return x >= &aabb[0] && x < &aabb[2] && y >= &aabb[1] && y < &aabb[3];
}

pub fn b_inside_a(a: &Rect, b: &Rect)->bool{
    rectpointcoll(a, &b[0], &b[1])||rectpointcoll(a, &b[2], &b[3])||rectpointcoll(a, &b[0], &b[3])||rectpointcoll(a, &b[2], &b[1])
}

pub fn rectrectcoll(a: &Rect, b: &Rect)->bool{
    b_inside_a(a, b)||b_inside_a(b, a)
}

pub fn middlepoint(rect: &Rect)->Vect{
    [(rect[0]+rect[2])/2.0, (rect[1]+rect[3])/2.0]
}

pub fn makesquare(x: f64, y: f64, s: f64)->Rect{
    [x,y,x+s,y+s]
}

pub fn makerect(x: f64, y: f64, w: f64, h: f64)->Rect{
    [x, y, x+w, y+h]
}

pub fn a_allin_b(a: &Rect, b: &Rect)->bool{
    rectpointcoll(b, &a[0], &a[1])&&rectpointcoll(b, &a[2], &a[3])
}

pub fn movrect(vx: &f64, vy: &f64, scalar: &f64, rect: &mut Rect){
    rect[0] += vx*scalar;
    rect[1] += vy*scalar;
    rect[2] += vx*scalar;
    rect[3] += vy*scalar;
}

pub fn fromang(alpha: f64, scalar: f64)->Vect{
    [scalar*alpha.cos(), scalar*alpha.sin()]
}

pub fn getang(vec: &Vect)->f64{
    if vec[0] == 0.0{
        if vec[1] > 0.0{
            return std::f64::consts::PI*0.5;
        }else if vec[1] < 0.0{
            return std::f64::consts::PI*1.5;
        }else{
            return 0.0;
        }
    }
    else if vec[0] < 0.0{
        return (vec[1]/vec[0]).atan()+std::f64::consts::PI;
    }
    return (vec[1]/vec[0]).atan()+2.0*std::f64::consts::PI;
}

pub fn add(v1: &mut Vect, v2: &Vect, scalar: f64){
    v1[0] += v2[0]*scalar;
    v1[1] += v2[1]*scalar;
}

pub fn to_xywh(r: &mut Rect){
    r[2] = r[2]-r[0];
    r[3] = r[3]-r[1];
}

