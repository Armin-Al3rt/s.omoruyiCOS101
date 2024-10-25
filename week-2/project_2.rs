fn main(){
	let t:f64=450_000.00;
	let m:f64=1_500_000.00;
	let h:f64=750_000.00;
	let d:f64=2_850_000.00;
	let a:f64=250_000.00;
	let tq:f64=2.0;
	let mq:f64=1.0;
	let hq:f64=3.0;
	let dq:f64=3.0;
	let aq:f64=1.0;

	let sum =(t*tq)+(m*mq)+(h*hq)+(d*dq)+(a*aq);
	let avg = sum/10.0;
	
	println!("the sum is {}and the avg is{}",sum,avg);

}