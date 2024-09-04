%% Load data and initialise parameters
um_per_mv = csvread('um_per_mv.csv');
um_per_mv = um_per_mv(1);
data=csvread('validation/1.csv');
time = data(2:end, 1);
voltage = data(2:end, 2) * 1000;  
len = length(time);

%% Filter
fs = 2400; 
cutoff_freq = 100;
[b,a] = butter(2, 100/(fs/2));
voltage = filter(b,a,voltage);

% for file_number = 2:5
%     filename = sprintf('calibration/%d.csv', file_number);
%     data = csvread(filename);
%     voltage = [voltage (data(2:len+1, 2) * 1000)];  
% end


voltage_avg = mean(voltage,2);
chunk_size = 10;
num_chunks = floor(length(voltage_avg) / chunk_size);

avg_values = zeros(1, num_chunks);
chunk_time = zeros(1, num_chunks);

%% Create chunks
for i = 1:num_chunks
    start_idx = (i-1) * chunk_size + 1;
    end_idx = i * chunk_size;
    chunk = voltage_avg(start_idx:end_idx);
    
    avg_values(i) = mean(chunk);
    chunk_time(i) = mean(time(start_idx:end_idx));
end


%% Get instron validation data;
instron = csvread('instron validation/Specimen_RawData_1.csv');
instron_time = instron(:, 1);
instron_displacement = instron(:, 2);

%% Starting point, where there's no voltage change
% Find index where we start seeing movement. 2 mV is just some arbitrary value.
i = avg_values > min(avg_values) + 2;

voltage = (max(avg_values(i)) - (avg_values(i)))'; % Shift values down
voltage = voltage(voltage < max(voltage) - 2); % Cut off anything less than 2 mV from the peak voltage.
shifted_time = chunk_time(i) - chunk_time(find(i, 1, "first"));

% Fit line
x = shifted_time(1:length(voltage))'; 
y = voltage*um_per_mv; % Convert mV to um
coeff = polyfit(x,y,1);
xfit = linspace(min(x), max(x), length(instron_displacement));
yfit = polyval(coeff, xfit);
% yfit has a y-intercept. Can see the data creating it, but instead of sanitising, we're introducing error by simply shifting the y-values down.
y_shifted = yfit - coeff(2);


csvwrite('validation/data-instron-lvdt.csv', [instron_displacement y_shifted']);
figure;
hold on; 
plot(instron_displacement, y_shifted, 'b.', 'MarkerSize', 15); % Plot training data.
xlabel('Instron displacement (um)'); ylabel('LVDT displacement (um)');

% Fit line on displacement x displacement data
coeff = polyfit(instron_displacement, y_shifted, 1);
yfit = polyval(coeff, xfit);
plot(xfit, yfit, 'r-', 'LineWidth', 2); % Plot fitted line.
legend("", sprintf('y = %.4fx + %.4f', coeff(1), coeff(2)));
grid on;
hold off;
